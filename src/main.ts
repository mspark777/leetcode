function arrayStringsAreEqual (word1: string[], word2: string[]): boolean {
  return word1.join('') === word2.join('')
}

interface Input {
  readonly word1: string[]
  readonly word2: string[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      word1: ['ab', 'c'],
      word2: ['a', 'bc']
    },
    {
      word1: ['a', 'cb'],
      word2: ['ab', 'c']
    },
    {
      word1: ['abc', 'd', 'defg'],
      word2: ['abcddefg']
    },
    {
      word1: ['abc', 'd', 'defg'],
      word2: ['abcddef']
    }
  ]

  for (const { word1, word2 } of inputs) {
    const result = arrayStringsAreEqual(word1, word2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
