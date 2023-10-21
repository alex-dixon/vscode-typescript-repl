// Imports

// First, observe numbers isn't defined here.
// @ts-ignore
numbers
// => 
// another-file.ts:1
// numbers;
// ^

// ReferenceError: numbers is not defined
//     at another-file.ts:1:1
//     at Script.runInContext (node:vm:141:12)
//     at Object.runInContext (node:vm:291:6)
//     at evaluate (/Users/alexdixon/.vscode/extensions/alexdixon.vscode-typescript-repl-0.0.2/dist/index.js:187162:20)

// Each file has its own space for names (namespace). 

// Import the definition for `numbers` from `a-file.ts`.
// @ts-ignore
import { numbers } from './a-file.ts'
// => [0, 1, 2, 3, 4]








