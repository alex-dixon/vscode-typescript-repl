/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface TransformOutput {
  code: string
  map?: string
}
export interface Neighbor {
  start: number
  end: number
  type: string
}
export interface TransformOutput {
  code: string
  isAsync: boolean
  map?: string
}
export interface EvaluableSpans {
  spans: Array<Neighbor>
}
export function evaluableSpans(source: string, target: number): EvaluableSpans
/**
 * Performs a transformation on the source string such that its output
 * is suitable for usage in a REPL environment.
 */
export function transformSync(source: string): TransformOutput