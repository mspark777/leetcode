import { minDeletions } from './solution'

interface Input {
  readonly s: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { s: 'aab' },
    { s: 'aaabbbcc' },
    { s: 'ceabaacb' }
  ]

  for (const input of inputs) {
    const result = minDeletions(input.s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
