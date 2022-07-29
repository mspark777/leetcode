import { findAndReplacePattern } from './solution'

interface Input {
  readonly words: string[]
  readonly pattern: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      words: ['abc', 'deq', 'mee', 'aqq', 'dkd', 'ccc'],
      pattern: 'abb'
    },
    {
      words: ['a', 'b', 'c'],
      pattern: 'a'
    }
  ]

  for (const input of inputs) {
    const result = findAndReplacePattern(input.words, input.pattern)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
