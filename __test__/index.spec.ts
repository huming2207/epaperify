import { readFile, writeFile } from 'fs/promises'

import test from 'ava'

import { to4Bpp, toMonochrome } from '../index'

test('Read test1 and convert to 4bpp', async (t) => {
  const input = await readFile('./__test__/test1.png')
  const output = await to4Bpp(input)
  t.true(output !== undefined)
  await writeFile('./__test__/test1-4bpp.png', output)
})

test('Read test1 and convert to monochrome', async (t) => {
  const input = await readFile('./__test__/test1.png')
  const output = await toMonochrome(input)
  t.true(output !== undefined)
  await writeFile('./__test__/test1-mono.png', output)
})

test('Read chickenfeet and convert to 4bpp', async (t) => {
  const input = await readFile('./__test__/chickenfeet.jpg')
  const output = await to4Bpp(input)
  t.true(output !== undefined)
  await writeFile('./__test__/chickenfeet-4bpp.png', output)
})

test('Read chickenfeet and convert to monochrome', async (t) => {
  const input = await readFile('./__test__/chickenfeet.jpg')
  const output = await toMonochrome(input)
  t.true(output !== undefined)
  await writeFile('./__test__/chickenfeet-mono.png', output)
})
