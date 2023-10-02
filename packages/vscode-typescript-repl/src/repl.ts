import * as vm from "vm"
import * as util from "util"
import {assignGlobal, createREPLErrorHandlers, createRequire} from "./vm"
import {Namespaces} from "./namespace"
import {tsToJS} from "./transpile"
import {Module} from "module"
import {v4 as uuid} from "uuid"
import * as R from "ramda"
import * as path from 'node:path'


export type REPLInput =
  {
    type: "expr";
    code: string;
    filename: string
  }
  | {
  type: "file"
}
export type PrintResult = {
  type: "print"
  input: REPLInput
  result: string
  filename: string
}
export type ErrorResult = {
  type: "error";
  text: string;
  filename: string;
  input?: REPLInput
}
export type REPLOutput =
  | {
  type: "value";
  result: any;
  filename: string;
  input?: REPLInput
}
  | ErrorResult | PrintResult


const log = console.log

const builtInBindings = new Set([
  "console",
  "require",
  "global",
  "setTimeout",
  "clearTimeout",
  "setInterval",
  "clearInterval",
  "setImmediate",
  "clearImmediate",
  "queueMicrotask",
  "performance",
  "$1",
  "$2",
])

export type REPL = {
  id: string
  // user-defined name for the repl
  name?: string
  currentNamespace: string
  namespaces: Namespaces
  handlers: {
    process: {
      // argh. can't import these
      uncaughtException: Function //UncaughtExceptionListener,
      unhandledRejection: Function //UnhandledRejectionListener
    }
  }
}

export type REPLs = Map<string, REPL>
export let repls: REPLs = new Map<string, REPL>()
// let replSessions = new Map<string, any>()

export type REPLOutboundEventMap = {
  "repl:output": REPLOutput
  "repl:namespace-changed": {
    namespace: string
  }
  "repl:namespace:defs:changed": {
    replId: string
    namespace: string
    added: Record<string, any>
    removed: string[]
    changed: Record<string, any>
  }
  "repl:reset": {
    replId: string
  }
}
export type REPLRequestResponseEventMap = {
  // creates a new empty repl
  "repl:new": {
    request: {
      name?: string
    }
    response: {
      replId: string
    }
  }
  // connects to an existing repl
  "repl:connect": {
    request: {
      replId: string
    }
    response: {
      replId: string
    } | {
      type: "error";
      message: string
    }
  }
  // call/response version of evaluate with option to silence all output (save
  // the value returned from the rpc call)
  "repl:evaluate-sync": {
    request: {
      replId: string
      filename: string
      code: string
      lineOffset: number
      columnOffset: number
      // whether to emit repl:output events, default is no
      broadcast?: boolean
    }
    response: REPLOutput
  }
}
export type REPLInboundEventMap = {
  "repl:set-current-namespace": {
    replId: string
    sessionId: string
    namespace: string
  }
  "repl:evaluate": {
    replId: string
    filename: string
    code: string
  }
  "repl:evaluate-namespace": {
    editorId: string;
    replId: string;
    namespace: string
  }
  "repl:reset": {
    replId: string
  }
  "repl:ns-unmap": {
    replId: string;
    namespace: string;
    symbol: string
  }
}

const replError = {
  repl_not_found: (replId: string, filename: string): ErrorResult =>
    ({
      type: "error",
      text: `No repl for replId ${replId}`,
      // todo. do not default
      filename: filename || "repl.ts",
    } as const),
}

process.on("uncaughtException", (e) => {
  console.error("uncaught exception from main?", e)
})
process.on("unhandledRejection", (e) => {
  console.error("unhandled rejection from main?", e)
})

// fixme. this runs on every evaluation. just looking for a way to print something
// nicely in js (without [Getter: 42] blah etc.)
const print = (x: unknown) =>
  util.inspect(
    // fixme.
    // this butchers promises and probably a bunch of other things. fooey.
    // typeof x === "object" && !Array.isArray(x) ? Object.assign({}, x) : x,
    x,
    {
      getters: true,
    }
  )

const api = {
  emit: (socket: any, topic: any, payload: any) => {
    console.log("emit", {socket, topic, payload})
  },
  broadcast: (topic: string, payload: any) => {
    console.log("broadcast", topic, payload)
  }
}

export const createREPL = (args: {
  name?: string,
  id?: string
},) => {
  log("repl:new")
  const replId = args.id || uuid()
  let namespaces = {}
  // todo. set unhandled error listeners for the repl..? or must be per namespace ctx?
  // make sure it always goes to the right socket(s).
  // may need to use broadcast on repl id topic or something
  const processErrorHandlers = createREPLErrorHandlers(api, replId)
  const repl = {
    id: replId,
    name: args.name,
    currentNamespace: "index.ts",
    namespaces: namespaces,
    handlers: {process: processErrorHandlers},
  }
  repls.set(replId, repl)
  log("created repl", repl)
  return repl
}
export type SetCurrentNamespaceInput = {
  replId: string
  sessionId: string
  namespace: string,
  __dirname: string
}
export const setCurrentNS = (args: SetCurrentNamespaceInput, socket: unknown) => {
  log("repl:set-current-namespace", args)
  const repl = repls.get(args.replId)
  if (!repl) {
    // @ts-ignore
    api.emit(socket, "repl:output", replError.repl_not_found(args.replId))
    return
  }
  repl.currentNamespace = args.namespace
  if (args.namespace in repl.namespaces) {
    return
  }
  const module = new Module(path.join(args.__dirname, args.namespace))
  const bindings = {
    require: createRequire(repl.namespaces, args.__dirname),
    module,
    __filename: args.namespace,
    __dirname: args.__dirname,//"<virtual-namespace>",
  }
  repl.namespaces[args.namespace] = {
    name: args.namespace,
    context: vm.createContext(bindings, {name: args.namespace}),
    defs: [],
  }
  api.emit(socket, "repl:namespace-changed", {namespace: args.namespace})
}

export type EvaluateSyncInput = {
  replId: string
  filename: string
  code: string
  lineOffset: number
  columnOffset: number
  // whether to emit repl:output events, default is no
  broadcast?: boolean
}
export type EvaluateSyncOutput = REPLOutput
export const evaluateSync = (args: EvaluateSyncInput, _socket: unknown): PrintResult | ErrorResult => {
  log("repl:evaluate-sync", args)
  const repl = repls.get(args.replId)
  if (!repl) {
    // todo. return consistent data
    return replError.repl_not_found(args.replId, args.filename) as unknown as ErrorResult
  }
  let namespace = repl.namespaces[args.filename]
  if (!namespace) {
    // panic / todo
    throw new Error("No such namespace")
  }

  let oldConsole
  if (!args.broadcast) {
    oldConsole = namespace.context.console
    const regularConsole = {
      debug: console.debug,
      log: console.log,
      info: console.info,
      warn: console.warn,
      error: console.error,
    }
    namespace.context.console = regularConsole
  }

  const jsCode = tsToJS(args.code)

  const result = vm.runInContext(jsCode.code, namespace.context, {
    columnOffset: args.columnOffset,
    lineOffset: args.lineOffset,
    filename: args.filename,
    displayErrors: true,
  })

  log("repl evaluate sync  run in context result", result)

  namespace.context["$2"] = namespace.context["$1"]
  namespace.context["$1"] = result

  if (oldConsole) {
    namespace.context.console = oldConsole
  }

  return {
    type: "print" as const,
    result: print(result),
    filename: args.filename,
    input: {
      type: "expr" as const,
      filename: args.filename,
      code: args.code,
    },
  }
}

type EvaluateInput = {
  replId: string
  filename: string
  __dirname: string,
  code: string
}

export const evaluate = async (args: EvaluateInput, socket: unknown): Promise<ErrorResult | PrintResult> => {
  log("repl:evaluate", args)

  const {replId} = args
  const repl = repls.get(replId)
  if (!repl) {
    // @ts-ignore
    return replError.repl_not_found(args.replId) as unknown as ErrorResult
  }

  const filenameOrNamespace = args.filename || repl.currentNamespace
  let namespace = repl.namespaces[filenameOrNamespace]

  // define it anyway...?
  if (!namespace) {
    console.debug("Expected namespace to be defined by now", {
      namespace: filenameOrNamespace,
    })

    const consoleSender = (...args: any[]) => {
      // todo. append to an array in order to return stuff maybe, or a cb/stream
      api.emit(socket, "repl:output", {
        type: "print",
        result: args.map((x) => print(x)).join(" "),
        filename: filenameOrNamespace,
        // todo. view should just print this absent input info
        input: {},
      })
    }
    const ctx = {
      exports: {},
      require: createRequire(repl.namespaces, args.__dirname),
      // there are 100 console methods. don't know if we could use an ES6 Proxy here since it goes to the vm...
      console: {
        debug: consoleSender,
        log: consoleSender,
        info: consoleSender,
        warn: consoleSender,
        error: consoleSender,
      },
    }
    assignGlobal(ctx)
    // todo. is this needed
    // @ts-ignore
    // ctx.__dirname=args.__dirname
    // // @ts-ignore
    // ctx.__filename=args.filename
    // const module = new Module(path.join(args.__dirname,args.filename))

    // // @ts-ignore
    // ctx.module=module


    // namespaces share nothing with the repl
    // could change this so process for example is shared, and attach
    // these at repl creation
    // first experimenting with each namespace context having own process variable
    // so each must have its own listener (applied exactly once)
    // the handlers are shared between all namespaces in this repl (broadcasts
    // an error to the repl by id, independent of namespace)
    // these should be added at namespace construction time (may want to centralize that one of these days)
    // there's no current need to maintain a reference to them or bind them to the namespace object since they
    // are the same fns defined at the repl level

    // @ts-expect-error I think process is defined?
    ctx.process.on("uncaughtException", repl.handlers.process.uncaughtException)
    // @ts-expect-error I think process is defined?
    ctx.process.on("unhandledRejection", repl.handlers.process.unhandledRejection)
    log("added repl process error handlers")

    const defs = Object.keys(ctx).filter((x) => !builtInBindings.has(x))
    namespace = repl.namespaces[filenameOrNamespace] = {
      name: filenameOrNamespace,
      defs,
      context: vm.createContext(ctx),
    }
  }

  let jsCode
  try {
    jsCode = tsToJS(args.code)
  } catch (e) {
    console.error("tsToJS error", e)
    return e
  }

  let ret
  try {
    const initialContext = Object.assign({}, namespace.context)
    let result
    if (jsCode.isAsync) {
      result = await vm.runInContext(jsCode.code, namespace.context, {
        // @ts-expect-error TODO
        columnOffset: args.from,
        // @ts-expect-error TODO
        lineOffset: args.from,
        filename: filenameOrNamespace,
        displayErrors: true,
      })
    } else {
      result = vm.runInContext(jsCode.code, namespace.context, {
        // @ts-expect-error TODO
        columnOffset: args.from,
        // @ts-expect-error TODO
        lineOffset: args.from,
        filename: filenameOrNamespace,
        displayErrors: true,
      })
    }

    log("run in context result", result)

    ret = {
      //replId,
      type: "print" as const,
      result: print(result),
      filename: filenameOrNamespace,
      input: {
        type: "expr",
        code: args.code,
        filename: filenameOrNamespace,
      },
    }
    api.emit(socket, "repl:output", ret)

    namespace.context["$2"] = namespace.context["$1"]
    namespace.context["$1"] = result

    const prevDefs = namespace.defs
    const defs = Object.keys(namespace.context).filter((k) => !builtInBindings.has(k))
    const added = R.difference(defs, prevDefs)
    const removed = R.difference(prevDefs, defs)
    let changed = {}
    defs.forEach((k) => {
      if (k in initialContext && namespace.context[k] !== initialContext[k]) {
        changed[k] = print(namespace.context[k])
      }
    })
    if (added.length || removed.length || Object.keys(changed).length) {
      api.emit(socket, "repl:namespace:defs:changed", {
        // replId,
        namespace: filenameOrNamespace,
        added: added.reduce((a, k) => {
          a[k] = print(namespace.context[k])
          return a
        }, {}),
        removed,
        changed,
      })
      namespace.defs = defs
    }

    // catch for error in the evaluated code
  } catch (e) {
    log("repl:output", e)
    // if (e instanceof SyntaxError) {
    //   if (
    //     e.message ===
    //     "await is only valid in async functions and the top level bodies of modules"
    //   ) {
    //     // let's help the kids out
    //     // todo. this requires ast transformation
    //     // const newCode = `{value: (async () => { ${args.code} })() } ;`
    //   }
    // }
    const error = {
      type: "error",
      text: print(e),
      filename: filenameOrNamespace,
      input: {
        type: "expr",
        code: args.code,
        filename: filenameOrNamespace,
      },
      // todo. ?
      // message: e.message,
      // stack: e.stack,
    }
    api.emit(socket, "repl:output", error)
    ret = error
  }
  return ret
}

type EvaluateNamespaceInput = {
  editorId: string;
  replId: string;
  namespace: string
}
const evaluateNamespace = (args: EvaluateNamespaceInput, socket: unknown) => {
  throw new Error("Not implemented for vscode")
  // todo. get the whole text of the selected file in the editor
  // pipe(
  //   files.getFile(args.namespace),
  //   TE.map((file) =>
  //     evaluate(
  //       {
  //         replId: args.replId || "test-repl-id",
  //         filename: args.namespace,
  //         code: file,
  //       },
  //       socket
  //     )
  //   ),
  //   TE.mapLeft((err) => {
  //     console.error("repl:evaluate-namespace", err)
  //     api.emit(socket, "repl:output", {
  //       type: "error",
  //       filename: args.namespace,
  //       text: err?.message,
  //     })
  //   })
  // )()
}

export type ResetInput = {
  replId: string
}
export const reset = (args: ResetInput, socket: unknown) => {
  const repl = repls.get(args.replId)
  if (!repl) {
    return
  }
  repl.namespaces = {}
  repl.currentNamespace = undefined
  api.emit(socket, "repl:reset", {replId: args.replId})
  return {replId: args.replId}
}

export type NSUnmapInput = {
  replId: string;
  namespace: string;
  symbol: string
}
export const nsUnmap = (args: NSUnmapInput, socket: unknown) => {
  const repl = repls.get(args.replId)
  if (!repl) {
    return
  }
  const namespace = repl.namespaces[args.namespace]
  if (!namespace) {
    console.error("No such namespace")
    return
  }
  if (!namespace.defs.includes(args.symbol)) {
    console.error("Symbol does not exist in namespace", {
      symbol: args.symbol,
      namespace: args.namespace,
    })
    return
  }
  namespace.defs = namespace.defs.filter((x) => x !== args.symbol)
  delete namespace.context[args.symbol]
  const output = {
    replId: args.replId,
    namespace: args.namespace,
    added: [],
    removed: [args.symbol],
    changed: [],
  }
  api.emit(socket, "repl:namespace:defs:changed", output)
  return {
    replId: args.replId,
    namespace: args.namespace,
    added: [],
    removed: [args.symbol],
    changed: [],
  }
}

// export const registerHandlers = (api: API, files: FilesSystem) => {
// api.handle("repl:new", )

// api.handle("repl:connect", async (args: { replId: string }, _socket) => {
//   log("repl:connect", args)
//   if (args.replId) {
//     const repl = repls.get(args.replId)
//     if (!repl) {
//       return replError.repl_not_found(args.replId)
//     }
//   }
//
//   // todo.
//   const replId = "test-repl-id" || uuid()
//   let namespaces = {}
//   // todo. set unhandled error listeners for the repl
//   // make sure it always goes to the right socket(s).
//   // may need to use broadcast on repl id topic or something
//   const processErrorHandlers = createREPLErrorHandlers(api, replId)
//   repls.set(replId, {
//     id: replId,
//     currentNamespace: "index.ts",
//     namespaces: namespaces,
//     handlers: {process: processErrorHandlers},
//   })
//
//   log("gonna return this", {replId})
//   return {replId}
// })

// api.on("repl:set-current-namespace", (args, socket) => {
// })

// api.handle("repl:evaluate-sync", async (args, _socket) => {
// })


// api.on("repl:evaluate", evaluate)
// api.on("repl:evaluate-namespace", )
// api.on("repl:reset", (args, socket) => )

// api.on("repl:ns-unmap", (args, socket) => )
// }
