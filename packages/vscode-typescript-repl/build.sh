#!/bin/sh


esbuild ./src/extension.ts --bundle --outfile=dist/index.js --external:vscode --format=cjs --platform=node --external:'*.node'
