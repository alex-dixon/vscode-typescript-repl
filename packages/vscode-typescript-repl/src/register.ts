import {addHook} from "pirates";
import {transformSync} from "swc-ts-repl-transpile";
import {logger} from "./logger";

const matcher = (filename) => {
  return true
}


const revert = addHook(
  (code, filename) => {
    try {
      return transformSync(code).code
    } catch (e) {
      logger.error('Require hook transformSync error', e)
    }
  },
  {
    exts: ['.ts'],
    matcher
  }
)
