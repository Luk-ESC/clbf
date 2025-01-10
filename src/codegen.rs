use std::path::PathBuf;

use codegen::ir::FuncRef;
use cranelift::prelude::*;
use cranelift_module::{DataDescription, FuncId, Init, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::optimisations::IrNode;

fn codegen_inner(
    recv: &mut impl Iterator<Item = IrNode>,
    builder: &mut FunctionBuilder,
    ptr: Option<Value>,
    putchar_func: FuncRef,
    getchar_func: FuncRef,
) -> Value {
    let mut ptr_val =
        ptr.unwrap_or_else(|| builder.block_params(builder.current_block().unwrap())[0]);

    while let Some(val) = recv.next() {
        println!("{val:?}");
        match val {
            IrNode::SetValue(x, offset) => {
                let value = builder.ins().iconst(types::I8, x as i64);
                builder.ins().store(MemFlags::new(), value, ptr_val, offset);
            }
            IrNode::ChangeValue(x, offset) => {
                let value = builder
                    .ins()
                    .load(types::I8, MemFlags::new(), ptr_val, offset);
                assert!(x < 255);

                let new_val = builder.ins().iadd_imm(value, x as i64);
                builder
                    .ins()
                    .store(MemFlags::new(), new_val, ptr_val, offset);
            }
            IrNode::DynamicChangeValue(-1, offset) => {
                let multiplier = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);
                let base = builder
                    .ins()
                    .load(types::I8, MemFlags::new(), ptr_val, offset);

                let new_value = builder.ins().isub(base, multiplier);

                builder
                    .ins()
                    .store(MemFlags::new(), new_value, ptr_val, offset);
            }
            IrNode::DynamicChangeValue(1, offset) => {
                let multiplier = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);
                let base = builder
                    .ins()
                    .load(types::I8, MemFlags::new(), ptr_val, offset);

                let new_value = builder.ins().iadd(base, multiplier);

                builder
                    .ins()
                    .store(MemFlags::new(), new_value, ptr_val, offset);
            }
            IrNode::DynamicChangeValue(x, offset) => {
                let multiplier = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);
                let base = builder
                    .ins()
                    .load(types::I8, MemFlags::new(), ptr_val, offset);

                let total_change = builder.ins().imul_imm(multiplier, x as i64);
                let new_value = builder.ins().iadd(base, total_change);

                builder
                    .ins()
                    .store(MemFlags::new(), new_value, ptr_val, offset);
            }
            IrNode::ChangePtr(x) => {
                ptr_val = builder.ins().iadd_imm(ptr_val, x as i64);
            }
            IrNode::PrintChar => {
                let char = builder
                    .ins()
                    .uload8(types::I32, MemFlags::new(), ptr_val, 0);
                builder.ins().call(putchar_func, &[char]);
            }
            IrNode::ReadChar => {
                let getchar_call = builder.ins().call(getchar_func, &[]);
                let char = builder.inst_results(getchar_call)[0];
                let char = builder.ins().ireduce(types::I8, char);

                builder.ins().store(MemFlags::new(), char, ptr_val, 0);
            }
            IrNode::LoopStart => {
                // Check block: The block that checks if the value at the current pointer is zero.
                // If it is, it jumps to the next block, otherwise it jumps to the loop block (does a new iteration)
                let check_block = builder.create_block();
                builder.append_block_param(check_block, types::I64);

                // Loop block: The block that contains the loop body. Always jumps back to the check block
                let loop_block = builder.create_block();
                builder.append_block_param(loop_block, types::I64);

                // Next block: The code after the loop.
                let next_block = builder.create_block();
                builder.append_block_param(next_block, types::I64);

                builder.insert_block_after(loop_block, builder.current_block().unwrap());
                builder.insert_block_after(check_block, loop_block);
                builder.insert_block_after(next_block, check_block);

                // Jump from current block to check block
                {
                    let value = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);
                    builder
                        .ins()
                        .brif(value, loop_block, &[ptr_val], next_block, &[ptr_val]);
                }

                builder.switch_to_block(check_block);
                // If the value at the current pointer is nonzero, jump to loop block, otherwise jump to next block
                {
                    let ptr_val = builder.block_params(check_block)[0];
                    let value = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);

                    builder
                        .ins()
                        .brif(value, loop_block, &[ptr_val], next_block, &[ptr_val]);
                }

                builder.switch_to_block(loop_block);
                {
                    let ptr_val = codegen_inner(recv, builder, None, putchar_func, getchar_func);

                    // Jump back to check block
                    builder.ins().jump(check_block, &[ptr_val]);
                }

                builder.switch_to_block(next_block);
                ptr_val = builder.block_params(next_block)[0];
                continue;
            }
            IrNode::LoopEnd => {
                return ptr_val;
            }
        }
    }

    ptr_val
}

fn declare_putchar(module: &mut ObjectModule) -> FuncId {
    let mut putchar_sig = module.make_signature();
    putchar_sig.params.push(AbiParam::new(types::I32)); // c
    putchar_sig.returns.push(AbiParam::new(types::I32)); // return value

    module
        .declare_function("putchar", Linkage::Import, &putchar_sig)
        .unwrap()
}

fn declare_getchar(module: &mut ObjectModule) -> FuncId {
    let mut getchar_sig = module.make_signature();
    getchar_sig.returns.push(AbiParam::new(types::I32)); // return value

    module
        .declare_function("getchar", Linkage::Import, &getchar_sig)
        .unwrap()
}

pub fn generate(
    mut recv: impl Iterator<Item = IrNode>,
    output: PathBuf,
    clif: Option<PathBuf>,
) -> std::io::Result<()> {
    let isa = cranelift_native::builder().unwrap();
    let mut builder = settings::builder();
    builder.set("opt_level", "speed").unwrap();

    let isa = isa.finish(settings::Flags::new(builder)).unwrap();

    let builder =
        ObjectBuilder::new(isa, "example", cranelift_module::default_libcall_names()).unwrap();
    let mut module = ObjectModule::new(builder);

    let data = module
        .declare_data("grid_memory", Linkage::Local, true, false)
        .unwrap();

    let putchar_func = declare_putchar(&mut module);
    let getchar_func = declare_getchar(&mut module);

    let mut context = module.make_context();
    // Define the signature of the `main` function
    context
        .func
        .signature
        .returns
        .push(AbiParam::new(types::I32));

    let mut func_ctx = FunctionBuilderContext::new();
    {
        let mut builder = FunctionBuilder::new(&mut context.func, &mut func_ctx);

        let block = builder.create_block();
        builder.switch_to_block(block);

        let local_putchar = module.declare_func_in_func(putchar_func, builder.func);
        let local_getchar = module.declare_func_in_func(getchar_func, builder.func);
        let local_data = module.declare_data_in_func(data, builder.func);

        let grid_ptr = {
            let grid_ptr = builder.ins().global_value(types::I64, local_data);
            builder.ins().iadd_imm(grid_ptr, 15000)
        };

        codegen_inner(
            &mut recv,
            &mut builder,
            Some(grid_ptr),
            local_putchar,
            local_getchar,
        );

        let zero = builder.ins().iconst(types::I32, 0);

        builder.ins().return_(&[zero]);
        builder.seal_all_blocks();
        builder.finalize();
    }

    if let Some(clif) = clif {
        std::fs::write(clif, context.func.to_string()).unwrap();
    }

    let id = module
        .declare_function("main", Linkage::Export, &context.func.signature)
        .unwrap();
    module.define_function(id, &mut context).unwrap();

    let mut data_description = DataDescription::new();
    data_description.init = Init::Zeros { size: 30000 };
    data_description.align = Some(1);
    module.define_data(data, &data_description).unwrap();

    // Write object to file (optional)
    let product = module.finish();
    let obj = product.emit().unwrap();
    std::fs::write(output, obj)
}
