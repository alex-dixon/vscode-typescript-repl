#!/bin/sh

find . -name "dist/*.node" -type f -delete

# for each file that ends in .node in the ../ts-repl-transpile directory, copy it to the dist directory
for f in ../ts-repl-transpile/*.node; do
    cp $f dist/
done
