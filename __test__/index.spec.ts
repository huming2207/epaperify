import { promises as fs } from 'fs'

import test from 'ava'

import {
  diffTwoQoiImages,
  imageToQoi,
  to4Bpp,
  to4BppWithTextMetadata,
  toMonochrome,
  toRgb4Bpp,
  toRgb4BppWithTextMetadata,
} from '../index'

test('Read test1 and convert to 4bpp', async (t) => {
  const input = await fs.readFile('./__test__/test1.png')
  const output = await to4Bpp(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/test1-4bpp.png', output)
})

test('Read test1 and convert to monochrome', async (t) => {
  const input = await fs.readFile('./__test__/test1.png')
  const output = await toMonochrome(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/test1-mono.png', output)
})

test('Read test1 and convert to RGB 4bpp', async (t) => {
  const input = await fs.readFile('./__test__/test1.png')
  const output = await toRgb4Bpp(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/test1-rgb-4bpp.png', output)
})

test('Read chickenfeet and convert to 4bpp', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await to4Bpp(input)
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
  const output = await toRgb4Bpp(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-rgb-4bpp.png', output)
})

test('Read chickenfeet and convert to 4bpp with tEXt', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await to4BppWithTextMetadata(input, { foo: 'bar', test: '567' }, false)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-4bpp-with-tEXt.png', output)
})

test('Read chickenfeet and convert to RGB 4bpp with tEXt', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await toRgb4BppWithTextMetadata(input, { foo: 'bar', rgb: 'true' }, false)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-rgb-4bpp-with-tEXt.png', output)
})

test('Diff chickenfeet and chickenfeet-4bpp', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const inputQoi = await imageToQoi(input)
  const bppQoi = await to4Bpp(input, 'qoi')
  const output = await diffTwoQoiImages(inputQoi, bppQoi)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-diff.bin', output)
})

test('Read chickenfeet and convert to 4bpp with QOI output', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await to4Bpp(input, 'qoi')
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-4bpp.qoi', output)
})

test('Read chickenfeet and convert to RGB 4bpp with QOI output', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await toRgb4Bpp(input, 'qoi')
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet-rgb-4bpp.qoi', output)
})

test('Read chickenfeet and convert to QOI', async (t) => {
  const input = await fs.readFile('./__test__/chickenfeet.jpg')
  const output = await imageToQoi(input)
  t.true(output !== undefined)
  await fs.writeFile('./__test__/chickenfeet.qoi', output)
})
