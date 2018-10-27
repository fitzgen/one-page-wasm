#!/usr/bin/env bash

set -ue

cd $(dirname "$0")
ROOT=$(pwd)

ONE_WASM_PAGE=65536

CI=${CI:-""}

if test ! -d node_modules || test "$CI" != ""; then
    npm install
fi

cd ./entries

JSON="["

for x in *; do
    if test ! -d "$x"; then
        continue
    fi

    echo "Building project $x"
    pushd "$x" > /dev/null

    # No cheating and trying to write custom stuff here.
    rm -rf ./pkg

    wasm-pack build > log.txt 2>&1 || {
        echo "Build for $x failed!"
        echo "=== log ==="
        cat log.txt
        exit 1
    }

    wasm_file=$(pwd)/$(ls pkg/*.wasm)
    js_file=$(pwd)/$(ls pkg/*.js)

    # Check that the wasm and JS is less than 64K!
    wasm_size=$(wc -c "$wasm_file" | awk '{ print $1 }')
    echo "    size of wasm: $wasm_size"
    js_size=$(wc -c "$js_file" | awk '{ print $1 }')
    echo "    size of js: $js_size"
    total_size=$(( $js_size + $wasm_size ))
    echo "    total size: $total_size"
    if [[ "$total_size" -gt "$ONE_WASM_PAGE" ]]; then
        echo "    Project $x is $total_size bytes -- that's bigger than $ONE_WASM_PAGE!"
        exit 1
    fi

    # Create the webpack page that pulls in the wasm and js.
    mkdir -p "../../built/$x"
    cd "../../built/$x/" > /dev/null

    cp "$ROOT/template/index.html" .
    cp "$ROOT/template/webpack.config.js" .
    cp "$ROOT/template/bootstrap.js" .
    cp "$wasm_file" .
    cp "$js_file" .

    # Make the bootstrap file import the correct module.
    underscore_x=${x//-/_}
    sed -i -e "s|XXX_MODULE|$underscore_x|g" bootstrap.js index.html
    sed -i -e "s|XXX_JS_SIZE|$js_size|g" bootstrap.js index.html
    sed -i -e "s|XXX_WASM_SIZE|$wasm_size|g" bootstrap.js index.html
    sed -i -e "s|XXX_TOTAL_SIZE|$total_size|g" bootstrap.js index.html
    sed -i -e "s|XXX_SOURCE|https://github.com/fitzgen/one-page-wasm/tree/master/entries/$x|g" bootstrap.js index.html

    # Build the bundle with webpack!
    "$ROOT/node_modules/.bin/webpack" --config webpack.config.js >> log.txt 2>&1 || {
        echo "webpack build for $x failed!"
        echo "=== log ==="
        cat log.txt
        exit 1
    }

    # Add the entry to the JSON.
    entry="{ \"name\": \"$x\", \"size\": { \"total\": $total_size, \"js\": $js_size, \"wasm\": $wasm_size } }"
    if [[ "$JSON" == "[" ]]; then
        JSON="$JSON"$'\n  '"$entry"
    else
        JSON="$JSON"$',\n  '"$entry"
    fi

    popd > /dev/null
done

JSON="$JSON"$'\n]'

echo "$JSON" > "../projects.json"

# Let CI deployment push built files.
if test "$CI" != ""; then
    rm -rf "$ROOT/node_modules"
    git rm "$ROOT/.gitignore"
fi
