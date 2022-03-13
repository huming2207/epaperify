# ePaperify

Framebuffer/image pre-processing library for e-Paper displays

## Usage

```ts
export function to4Bpp(
  image: Buffer,
  format?: string | undefined | null,
  signal?: AbortSignal | undefined | null,
): Promise<Buffer>
```

Converts any image to 4-bit-per-pixel grayscale, with dithering to emulate 8-bit-per-pixel

- Parameters:
  - `image`: image buffer, can be any image format supported by Rust `image` library, e.g. PNG, JPG, TIFF, WebP...
  - `format`: format string, e.g. "png", "jpg"; by default it's "png"
  - `signal`: signal of `AbortController`

```ts
export function toMonochrome(image: Buffer): Promise<Buffer>
```

Converts any image to 1-bit-per-pixel grayscale, with dithering to emulate 8-bit-per-pixel

- Parameters:
  - `image`: image buffer, can be any image format supported by Rust `image` library, e.g. PNG, JPG, TIFF, WebP...
  - `format`: format string, e.g. "png", "jpg"; by default it's "png"
  - `signal`: signal of `AbortController`
