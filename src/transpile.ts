import { TransformOutput, transformSync } from "swc-ts-repl-transpile"

export const tsToJS = (code: string): TransformOutput => {
  const result = transformSync(code)
  console.log("transform result", result)
  return result
}
