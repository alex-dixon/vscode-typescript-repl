# vscode-typescript-repl
![image](https://github.com/alex-dixon/vscode-typescript-repl/assets/9045165/48e5f343-a5a6-46ad-8e6f-ca55db898ee1)


[![Visual Studio Marketplace](https://vsmarketplacebadges.dev/version/AlexDixon.vscode-typescript-repl.svg)](https://marketplace.visualstudio.com/items?itemName=AlexDixon.vscode-typescript-repl)


This is a plugin for VSCode that provides an interactive programming experience for the TypeScript programming language. 


## Features

### Develop faster

### Reason locally
Debugging one function shouldn't require running the entire program. 

### Explore easily
It should not be a chore to do good things. Running code is ultimately how we know it works. The ability to do this easier and faster has compunding effects on software development. 

### Combine getting it working with it working
An integrated project REPL environment lets you write and evaluate code one moment and ship it the next.

### Code in files

### Evaluate any code in any file

### Interact with your software


## Requirements

macOS

## Extension Settings

* `typescript-repl.evaluate`: Evaluate the current selection.
* `typescript-repl.evaluate-file`: Evaluate the current file.


## FAQ
**Q**: How do I run shell commands?

**A**: Either:
- Set cwd when running exec* commands:
```typescript
import {execSync} from "node:child_process"

execSync("ls", {cwd: "/some/path"})
```
- Set the root directory for the node process by evaluating `process.chdir`:
```typescript
// In any file
process.chdir("/some/path")
process.cwd() // "/some/path"
```


