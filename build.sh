mkdir -p bin/


cargo b -r && \
    echo "## Built compiler successfully" && \
    time ./target/release/clbf test.bf -o bin/output.o
    echo "## Compiled successfully" && \
    ld -o bin/compiled bin/output.o -lc  /usr/lib/crti.o /usr/lib/crt1.o /usr/lib/crtn.o  --dynamic-linker=/lib64/ld-linux-x86-64.so.2 && \
    echo "## Linked successfully" && \
    time ./bin/compiled && \
    echo "## Ran successfully"
