import { numMatchingSubseq } from './solution'

interface Input {
  readonly s: string
  readonly words: string[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      s: 'abcde',
      words: ['a', 'bb', 'acd', 'ace']
    },
    {
      s: 'dsahjpjauf',
      words: ['ahjpjau', 'ja', 'ahbwzgqnuk', 'tnmlanowax']
    }
  ]

  for (const input of inputs) {
    const result = numMatchingSubseq(input.s, input.words)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
