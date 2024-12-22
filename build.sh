mkdir -p bin/

time cargo r && \
    echo "## Compiled successfully" && \
    ld -o bin/compiled bin/output.o -lc /usr/lib/crt1.o --dynamic-linker=/lib64/ld-linux-x86-64.so.2 && \
    echo "## Linked successfully" && \
    ./bin/compiled && \
    echo "## Ran successfully"
