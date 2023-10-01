// Imports

// Each file has its own space for names (namespace). 
// `numbers` isn't defined in this file.
// @ts-ignore
numbers
// =>
// ReferenceError: numbers is not defined
//  at another-file.ts:1:1
//  at Script.runInContext (node:vm:141:12)
//  at Object.runInContext (node:vm:291:6)

// Import the definition for `numbers` from `a-file.ts`.
import { numbers } from 'ns:a-file.ts'
// => [0, 1, 2, 3, 4]








