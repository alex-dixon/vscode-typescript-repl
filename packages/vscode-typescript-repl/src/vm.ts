import {Module} from "module"
import * as vm from "vm"
import * as util from "util"
import {Namespaces} from "./namespace"
import * as path from 'node:path'
import * as fs from "fs";

const isNamespaceModuleIdent = (id: string) => id.startsWith("ns:")

export const createRequire = (namespaces: Namespaces, __dirname: string) => {
  const baseRequire = Module.createRequire(__dirname)
  let requestedAbsolutePathToModuleResolvedAbsolutePath = {}
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
    // resolve relative paths outside extension-land (relative to the file being evaluated)
    if (id.startsWith("./")) {
      console.log("Requiring a relative filesystem path thing?", {
        id, __dirname,
        // fromLibraryRequire: myRequire.resolve(id),
        fullPath: path.join(__dirname, id)
      })
      // TODO: tempting as it would be to just implement the logic here directly and be done with it,
      //  there's the potential for a lot of perf loss here (no caching, syscalls, etc).
      // at the end of the day this should just be a normal ts-node require.
      // it should be patched by pirates/addHook in ./register.ts file already. but it's not,
      // probably because this uses Module.createRequire

      const requested = path.join(__dirname, id)

      // if cached return the require result
      if (requestedAbsolutePathToModuleResolvedAbsolutePath[requested]) {
        return baseRequire(requestedAbsolutePathToModuleResolvedAbsolutePath[requested])
      }
      // try to resolve it ourselves...:)
      let resolved = requested
      const toTry = requested.endsWith(".ts")
        ? [requested]
        : [requested + ".ts", path.join(requested, "index.ts")]
      for (const p of toTry) {
        if (fs.existsSync(p)) {
          resolved = p
          requestedAbsolutePathToModuleResolvedAbsolutePath[requested] = resolved
          break
        }
      }
      console.log('resolved', resolved)
      return baseRequire(resolved)
    }
    return baseRequire(id)
  }
  Object.defineProperty(require, "name", {value: "require"})
  require.main = baseRequire.main
  require.cache = baseRequire.cache
  require.resolve = baseRequire.resolve
  // this may be needed, don't know yet
  // @ts-ignore
  // require.resolve = (id: string, options?: {
  //   paths?: string[];
  // }) => {
  //   if (id.startsWith("./")) {
  //     // to
  //     return path.resolve(path.join(__dirname, id) + ".ts")
  //   }
  //   return baseRequire.resolve(id, options)
  // }
  // require.resolve.paths = baseRequire.resolve.paths
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
    if (!globalBuiltinNames.has(k)) {
      Object.defineProperty(context, k, {
        // @ts-expect-error
        __proto__: null,
        ...Object.getOwnPropertyDescriptor(_global, k),
      })
    }
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
