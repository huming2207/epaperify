/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function to4Bpp(image: Buffer): Promise<Buffer>
export function to4BppAbortable(image: Buffer, signal: AbortSignal): Promise<Buffer>
export function toMonochrome(image: Buffer): Promise<Buffer>
export function toMonochromeAbortable(image: Buffer, signal: AbortSignal): Promise<Buffer>
export function plus100(input: number): number
