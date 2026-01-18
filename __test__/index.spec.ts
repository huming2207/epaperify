import { promises as fs } from 'fs'

import test from 'ava'

import {
  diffTwoQoiImages,
  toDitheredGreyImage,
  toDitheredGreyWithPngTxt,
  toMonochrome,
  toPng,
  toRgbImage,
} from '../index'

test('Read test1 and convert to 4bpp greyscale', async (t) => {
  const input = await fs.readFile('./__test__/test1.png')
  const output = await toDitheredGreyImage(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/test1-4bpp.png', output)
})

test('Read test1 and convert to monochrome', async (t) => {
  const input = await fs.readFile('./__test__/test1.png')
  const output = await toMonochrome(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/test1-mono.png', output)
})

test('Read test1 and convert to RGB image', async (t) => {
  const input = await fs.readFile('./__test__/test1.png')
  const output = await toRgbImage(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/test1-rgb.png', output)
})

test('Read chickenfeet and convert to 4bpp', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await toDitheredGreyImage(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-4bpp.png', output)
})

test('Read chickenfeet and convert to monochrome', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await toMonochrome(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-mono.png', output)
})

test('Read chickenfeet and convert to RGB 4bpp', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await toRgbImage(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-rgb.png', output)
})

test('Read chickenfeet and convert to 4bpp with tEXt', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await toDitheredGreyWithPngTxt(input, { foo: 'bar', test: '567' }, false)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-4bpp-with-tEXt.png', output)
})

test('Read chickenfeet and convert to RGB 4bpp with tEXt', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await toPng(input, { foo: 'bar', rgb: 'true' }, false)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-rgb-4bpp-with-tEXt.png', output)
})

test('Diff countdown', async (t) => {
  const oldPng = await fs.readFile('./__test__/countdown-56.png')
  const newPng = await fs.readFile('./__test__/countdown-18.png')
  const oldQoi = await toRgbImage(oldPng, 'qoi')
  const newQoi = await toRgbImage(newPng, 'qoi')
  const output = await diffTwoQoiImages(newQoi, oldQoi)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/countdown-diff.bin', output)
  await fs.writeFile('./__test__/countdown-56.qoi', oldQoi)
  await fs.writeFile('./__test__/countdown-18.qoi', newQoi)
})

test('Convert JPG to RGB QOI image', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await toRgbImage(input, 'qoi')
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-rgb.qoi', output)
})
