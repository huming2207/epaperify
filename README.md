# ePaperify

High-performance image pre-processing library for e-Paper displays, built with Rust and NAPI-RS.

This library provides fast native bindings for Node.js to perform common image manipulations required for e-Paper displays, such as dithering, colorspace conversion, and differential updates using the QOI format.

## Installation

```bash
npm install @huming2207/epaperify
# or
yarn add @huming2207/epaperify
```

## Features

- **Dithering**: Convert images to 4-bit (16 levels) grayscale or 1-bit monochrome with high-quality dithering.
- **QOI Support**: Fast encoding to [QOI (Quite OK Image)](https://qoiformat.org/) format.
- **Differential Updates**: efficient calculation of differences between two QOI images to support partial screen updates.
- **Format Conversion**: Convert between common formats (PNG, JPEG, WebP, etc.) and raw buffers.
- **Metadata**: Add text chunks (metadata) to PNG files.
- **Performance**: Built with Rust for native performance.
- **Asynchronous**: Non-blocking API with `AbortSignal` support.

## Usage

### Import

```ts
import { toDitheredGreyImage, toMonochrome, toQoi, diffTwoQoiImages, toPng, QoiChannels } from '@huming2207/epaperify'
import fs from 'fs/promises'
```

### Convert to 4-bit Grayscale (Dithered)

Converts an image to 16-level grayscale with dithering. Useful for 4bpp e-Paper displays.

```ts
const input = await fs.readFile('input.jpg')
// Returns a Buffer containing a PNG (default) image with 4-bit palette
const dithered = await toDitheredGreyImage(input, 'png')
await fs.writeFile('output_4bpp.png', dithered)
```

### Convert to Monochrome (1-bit)

Converts an image to black and white with dithering. Useful for standard 1bpp e-Paper displays.

```ts
const input = await fs.readFile('input.jpg')
const mono = await toMonochrome(input, 'png')
await fs.writeFile('output_1bpp.png', mono)
```

### QOI Conversion & Diffing

The QOI format is extremely fast to encode/decode, making it ideal for embedded devices. This library can compute the difference between two QOI images, generating a patch image (transparent where pixels match) for partial updates.

```ts
const img1 = await fs.readFile('frame1.png')
const img2 = await fs.readFile('frame2.png')

// Convert both to QOI
const qoi1 = await toQoi(img1, QoiChannels.Rgb)
const qoi2 = await toQoi(img2, QoiChannels.Rgb)

// Calculate difference: result is a QOI buffer where unchanged pixels are transparent
// This is perfect for sending only changed pixels to an e-Paper display
const diffQoi = await diffTwoQoiImages(qoi2, qoi1)

await fs.writeFile('diff.qoi', diffQoi)
```

### Advanced PNG Export

Convert an image to PNG with custom metadata (Text Chunks).

```ts
const input = await fs.readFile('photo.jpg')
const pngWithMeta = await toPng(
  input,
  {
    Author: 'John Doe',
    Description: 'Sunset',
  },
  true, // compress text chunks
)
```

## API Reference

### `toDitheredGreyImage`

```ts
toDitheredGreyImage(
  image: Buffer,
  format?: string | null, // 'png', 'jpeg', etc. Default: 'png'
  signal?: AbortSignal | null
): Promise<Buffer>
```

### `toMonochrome`

```ts
toMonochrome(
  image: Buffer,
  format?: string | null,
  signal?: AbortSignal | null
): Promise<Buffer>
```

### `toQoi`

```ts
toQoi(
  image: Buffer,
  channels?: QoiChannels | null, // QoiChannels.Rgb (3) or QoiChannels.Rgba (4)
  signal?: AbortSignal | null
): Promise<Buffer>
```

### `diffTwoQoiImages`

```ts
diffTwoQoiImages(
  newImage: Buffer, // Buffer containing QOI data
  oldImage: Buffer, // Buffer containing QOI data
  signal?: AbortSignal | null
): Promise<Buffer>
```

### `toPng`

```ts
toPng(
  image: Buffer,
  textChunks?: Record<string, string> | null,
  compressedText?: boolean | null,
  bestCompression?: boolean | null,
  signal?: AbortSignal | null
): Promise<Buffer>
```

### `toRgbImage`

```ts
toRgbImage(
  image: Buffer,
  format?: string | null,
  signal?: AbortSignal | null
): Promise<Buffer>
```

## License

MIT
