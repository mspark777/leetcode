import { minMoves2 } from './solution'

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { nums: [1, 2, 3] },
    { nums: [1, 10, 2, 9] }
  ]

  for (const input of inputs) {
    const result = minMoves2(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
