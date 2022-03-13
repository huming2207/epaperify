/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function to4Bpp(
  image: Buffer,
  format?: string | undefined | null,
  signal?: AbortSignal | undefined | null,
): Promise<Buffer>
export function toMonochrome(
  image: Buffer,
  format?: string | undefined | null,
  signal?: AbortSignal | undefined | null,
): Promise<Buffer>
export const VERSION: string
