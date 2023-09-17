import {addHook} from "pirates";
import * as ts from "typescript";

const matcher = (filename) => {
  console.log("matcher", filename)
  // TODO: Implement your logic here
  return filename.endsWith('.ts');
}

console.log("gonna add hook")
const revert = addHook(
  (code, filename) => {
    try {
      let result = ts.transpileModule(code, {fileName: filename, compilerOptions: {module: ts.ModuleKind.CommonJS}});

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
      console.log('result', result)
      return result.outputText
    } catch
      (e) {
      console.log('whattt', e)
    }
  },
  {
    exts: ['.ts'], matcher
  }
)
