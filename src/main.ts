import { kInversePairs } from './solution'

interface Input {
  readonly n: number
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      n: 3,
      k: 0
    },
    {
      n: 3,
      k: 1
    }
  ]

  for (const input of inputs) {
    const result = kInversePairs(input.n, input.k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
