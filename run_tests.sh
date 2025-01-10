#!/bin/bash

cargo b -r || exit 1
echo "# Built compiler successfully"

compiler=$(realpath ./target/release/clbf)

bless=false

if [[ $1 == "-b" ]]; then
    bless=true
fi


cd tests
for dir in *; do
    if [[ -f $dir ]]; then
        continue
    fi

    cd $dir || exit 2
    echo
    echo "## Test: $dir"

    file=$dir.bf
    if [[ ! -f $file ]]; then
        echo "File $file does not exist"
        exit 1
    fi

    # get temp dir
    tmpdir=$(mktemp -d)

    echo "### Compiling $file"
    $compiler $file -o $tmpdir/out.o -r $tmpdir/out.rs -c $tmpdir/out.clif || exit 1
    echo "### Compiled $file"

    echo "### Linking $tmpdir/out.o"
    ld -o $tmpdir/$dir $tmpdir/out.o -lc  /usr/lib/crti.o /usr/lib/crt1.o /usr/lib/crtn.o
    echo "### Linked $tmpdir/out.o"

    echo "### Running $tmpdir/$dir"
    $tmpdir/$dir > $tmpdir/out.txt || exit 3

    if ! $bless; then
        if ! diff $dir.txt $tmpdir/out.txt  --color=always; then
            echo "### FAIL: $dir output changed"

            rm -rf $tmpdir
            exit 1
        fi

        # Make sure that tmpdir/out.rs is the same as $dir.rs
        if ! diff $dir.rs $tmpdir/out.rs --color=always; then
            echo "### FAIL: $dir rust output changed"

            rm -rf $tmpdir
            exit 1
        fi

        # Make sure that tmpdir/out.clif is the same as $dir.clif
        if ! diff $dir.clif $tmpdir/out.clif --color=always; then
            echo "### FAIL: $dir clif output changed"

            rm -rf $tmpdir
            exit 1
        fi
    else
        cp $tmpdir/out.rs $dir.rs
        cp $tmpdir/out.clif $dir.clif
        cp $tmpdir/out.txt $dir.txt
    fi

    rm -rf $tmpdir
    cd ..
done
