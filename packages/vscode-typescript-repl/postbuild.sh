#!/bin/sh

# for each file that ends in .node in the ../ts-repl-transpile directory, copy it to the dist directory
for f in ../ts-repl-transpile/*.node; do
    cp $f dist/
done

#
## make a temporary directory
#mkdir -p temporarydirectory
#cd temporarydirectory || exit
#
#
## Write a minimal package.json file
#echo '{"name": "temporarydirectory", "version": "1.0.0", "description": "", "main": "index.js", "scripts": {"test": "echo \"Error: no test specified\" && exit 1"}, "keywords": [], "author": "", "license": "ISC"}' > package.json
#npm i @swc/core@1.3.83
#cp -r node_modules ../dist
#cd ..
#rm -rf temporarydirectory
