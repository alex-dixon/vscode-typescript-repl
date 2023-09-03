import * as vm from "vm"

export type Namespaces = Record<
  string,
  {
    name: string
    defs: string[]
    context: vm.Context
  }
>
export const namespaceToTSFilename = (ns: string) => ns
// `/${ns}.ts`
