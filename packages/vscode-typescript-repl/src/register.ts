import {addHook} from "pirates";
import * as ts from "typescript";

const matcher = (filename) => {
  console.log("matcher", filename)
  // TODO: Implement your logic here
  return true
  // return filename.endsWith('.ts');
}

console.log("gonna add hook")
const revert = addHook(
  (code, filename) => {
    try {
      let result = ts.transpileModule(code, {fileName: filename, compilerOptions: {
        module: ts.ModuleKind.CommonJS,
          target: ts.ScriptTarget.ES2022,

        }});

      // const result = swc.transformSync(code, {
      //   filename,
      //   sourceFileName: filename,
      //   module: {type:'commonjs'},
      //   jsc: {
      //     loose:true,
      //
      //     parser: {syntax: "typescript",},
      //     target: "es2019",
      //   }
      // })
      console.log('transpile typescript result', result)
      return result.outputText
    } catch (e) {
      console.log('whattt', e)
    }
  },
  {
    exts: ['.ts'], matcher
  }
)
