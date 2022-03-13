# ePaperify

Framebuffer/image pre-processing library for e-Paper displays

## Usage


```ts
export function to4Bpp(image: Buffer): Promise<Buffer>
```

Converts any image to 4-bit-per-pixel grayscale, with dithering to emulate 8-bit-per-pixel 

```ts
export function to4BppAbortable(image: Buffer, signal: AbortSignal): Promise<Buffer>
```

Converts any image to 4-bit-per-pixel grayscale, with dithering to emulate 8-bit-per-pixel, with `AbortController` signal support

```ts
export function toMonochrome(image: Buffer): Promise<Buffer>
```

Converts any image to 1-bit-per-pixel grayscale, with dithering to emulate 8-bit-per-pixel 

```ts
export function toMonochromeAbortable(image: Buffer, signal: AbortSignal): Promise<Buffer>
```

Converts any image to 1-bit-per-pixel grayscale, with dithering to emulate 8-bit-per-pixel, with `AbortController` signal support