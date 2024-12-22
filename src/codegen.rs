use std::sync::mpsc::Receiver;

use crate::preopts::IrNode;
use codegen::ir::FuncRef;
use cranelift::prelude::*;
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use types::I64;

fn codegen_inner(
    recv: &mut impl Iterator<Item = IrNode>,
    builder: &mut FunctionBuilder,
    ptr: Variable,
    putchar_func: FuncRef,
    getchar_func: FuncRef,
) {
    while let Some(val) = recv.next() {
        println!("{val:?}");
        match val {
            IrNode::Incr(x) => {
                let ptr_val = builder.use_var(ptr);
                let value = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);
                assert!(x < 255);
                let new_val = builder.ins().iadd_imm(value, x as i64);
                builder.ins().store(MemFlags::new(), new_val, ptr_val, 0);
            }
            IrNode::Decr(x) => {
                let ptr_val = builder.use_var(ptr);
                let value = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);
                assert!(x < 255);
                let new_val = builder.ins().iadd_imm(value, -(x as i64));
                builder.ins().store(MemFlags::new(), new_val, ptr_val, 0);
            }
            IrNode::IncrPtr(x) => {
                let ptr_val = builder.use_var(ptr);
                let new_val = builder.ins().iadd_imm(ptr_val, x as i64);
                builder.def_var(ptr, new_val);
            }
            IrNode::DecrPtr(x) => {
                let ptr_val = builder.use_var(ptr);
                let new_val = builder.ins().iadd_imm(ptr_val, -(x as i64));
                builder.def_var(ptr, new_val);
            }
            IrNode::PrintChar => {
                let ptr_val = builder.use_var(ptr);
                let char = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);
                let char = builder.ins().uextend(types::I32, char);
                let putchar_call = builder.ins().call(putchar_func, &[char]);
                let _putchar_result = builder.inst_results(putchar_call)[0];
            }
            IrNode::ReadChar => {
                let ptr_val = builder.use_var(ptr);
                let getchar_call = builder.ins().call(getchar_func, &[]);
                let char = builder.inst_results(getchar_call)[0];
                let char = builder.ins().ireduce(types::I8, char);

                builder.ins().store(MemFlags::new(), char, ptr_val, 0);
            }
            IrNode::LoopStart => {
                // create new block and switch to it
                let loop_block = builder.create_block();
                let skip_block = builder.create_block();
                let next_block = builder.create_block();

                // Jump from current block to skip block
                builder.ins().jump(skip_block, &[]);

                builder.switch_to_block(skip_block);
                // If the value at the current pointer is nonzero, jump to loop block, otherwise jump to next block
                {
                    let ptr_val = builder.use_var(ptr);
                    let value = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);

                    builder.ins().brif(value, loop_block, &[], next_block, &[]);
                }

                builder.switch_to_block(loop_block);
                codegen_inner(recv, builder, ptr, putchar_func, getchar_func);

                // Add branch instruction to loop block
                let ptr_val = builder.use_var(ptr);
                let value = builder.ins().load(types::I8, MemFlags::new(), ptr_val, 0);

                // If non-zero, jump back to loop start
                // If zero, jump to next loop
                builder.ins().brif(value, loop_block, &[], next_block, &[]);

                builder.switch_to_block(next_block);
                continue;
            }
            IrNode::LoopEnd => {
                return;
            }
        }
    }
}

fn declare_calloc(module: &mut ObjectModule) -> FuncId {
    let mut calloc_sig = module.make_signature();
    calloc_sig.params.push(AbiParam::new(types::I64)); // num
    calloc_sig.params.push(AbiParam::new(types::I64)); // size
    calloc_sig.returns.push(AbiParam::new(types::I64)); // return pointer

    module
        .declare_function("calloc", Linkage::Import, &calloc_sig)
        .unwrap()
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

pub fn generate(recv: Receiver<IrNode>) {
    let isa = cranelift_native::builder()
        .unwrap()
        .finish(settings::Flags::new(settings::builder()))
        .unwrap();

    let builder =
        ObjectBuilder::new(isa, "example", cranelift_module::default_libcall_names()).unwrap();
    let mut module = ObjectModule::new(builder);

    let calloc_func = declare_calloc(&mut module);
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
        let grid = Variable::new(0);
        let ptr = Variable::new(1);

        builder.declare_var(grid, I64);
        builder.declare_var(ptr, I64);

        let block = builder.create_block();
        builder.switch_to_block(block);

        let local_calloc = module.declare_func_in_func(calloc_func, builder.func);
        let local_putchar = module.declare_func_in_func(putchar_func, builder.func);
        let local_getchar = module.declare_func_in_func(getchar_func, builder.func);

        {
            let one = builder.ins().iconst(I64, 1);
            let thirty_thousand = builder.ins().iconst(I64, 30000);

            let calloc_call = builder.ins().call(local_calloc, &[one, thirty_thousand]);
            let value = builder.inst_results(calloc_call)[0];

            builder.def_var(grid, value);
            builder.def_var(ptr, value);
        };

        let mut recv = recv.iter();
        codegen_inner(&mut recv, &mut builder, ptr, local_putchar, local_getchar);

        let zero = builder.ins().iconst(types::I32, 0);

        builder.ins().return_(&[zero]);
        builder.seal_all_blocks();
        builder.finalize();
    }

    let id = module
        .declare_function("main", Linkage::Export, &context.func.signature)
        .unwrap();
    module.define_function(id, &mut context).unwrap();

    // Write object to file (optional)
    let product = module.finish();
    let obj = product.emit().unwrap();
    std::fs::write("bin/output.o", obj).unwrap();
}
