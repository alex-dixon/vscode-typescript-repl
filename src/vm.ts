import { Module } from "module"
import * as vm from "vm"
import * as util from "util"
import { Namespaces } from "./namespace"

const isNamespaceModuleIdent = (id: string) => id.startsWith("ns:")
export const createRequire = (namespaces: Namespaces,__dirname:string) => {
  const baseRequire = Module.createRequire(__dirname)
  // @ts-ignore
  let require: NodeJS.Require = (id: string) => {
    if (isNamespaceModuleIdent(id)) {
      const nsId = id.slice(3)
      const namespace = namespaces[nsId]
      if (!namespace) {
        throw new Error(`Namespace not found: ${nsId}`)
      }
      return namespace.context.exports || {}
    }
    return baseRequire(id)
  }
  Object.defineProperty(require, "name", { value: "require" })
  require.main = baseRequire.main
  require.cache = baseRequire.cache
  require.resolve = baseRequire.resolve
  require.extensions = baseRequire.extensions
  return require
}
const globalBuiltinNames = new Set(
  vm.runInNewContext("Object.getOwnPropertyNames(globalThis)")
)
export const assignGlobal = (context: vm.Context) => {
  let _global = global
  // delete _global.global
  // guard against possibility our global has more than the vm
  const globalDescriptors = Object.getOwnPropertyNames(_global)
  globalDescriptors.forEach((k) => {
    if (!globalBuiltinNames.has(k))
      Object.defineProperty(context, k, {
        // @ts-expect-error
        __proto__: null,
        ...Object.getOwnPropertyDescriptor(_global, k),
      })
  })
}

export const createREPLErrorHandlers = (api: any, replId: string) => {
  return {
    uncaughtException: (e, origin) => {
      console.error(`Uncaught exception for repl id ${replId}`, e, origin)
      api.broadcast(`repl:${replId}:uncaughtException`, {
        error: util.inspect(e),
      })
    },
    unhandledRejection: (e) => {
      console.error(`Uncaught exception for repl id ${replId}`, e)
      api.broadcast(`repl:${replId}:unhandledRejection`, {
        error: util.inspect(e),
      })
    },
  }
}
