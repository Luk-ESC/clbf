#!/bin/bash

cargo b -r || exit 1
echo "# Built compiler successfully"

compiler=$(realpath ./target/release/clbf)

bless=false

if [[ $1 == "-b" ]]; then
    bless=true
fi

myglob="*"

if [[ -n $2 ]]; then
    myglob=$2
fi

cd tests
for dir in $myglob; do
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
    $compiler $file -o $tmpdir/$dir -r $tmpdir/out.rs -c $tmpdir/out.clif || exit 1
    echo "### Compiled $file"

    echo "### Running $tmpdir/$dir"
    $tmpdir/$dir > $tmpdir/out.txt || exit 3

    if ! $bless; then
        if ! diff $dir.txt $tmpdir/out.txt  --color=always; then
            echo "### FAIL: $dir output changed"
            read

            rm -rf $tmpdir
            exit 1
        fi

        # Make sure that tmpdir/out.rs is the same as $dir.rs
        if ! diff $dir.rs $tmpdir/out.rs --color=always; then
            echo "### FAIL: $dir rust output changed"
            read

            rm -rf $tmpdir
            exit 1
        fi

        # Make sure that tmpdir/out.clif is the same as $dir.clif
        if ! diff $dir.clif $tmpdir/out.clif --color=always; then
            echo "### FAIL: $dir clif output changed"
            read

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
