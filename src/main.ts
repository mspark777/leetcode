import { searchRange } from './solution'

interface Input {
  readonly nums: number[]
  readonly target: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [5, 7, 7, 8, 8, 10], target: 8
    },
    {
      nums: [5, 7, 7, 8, 8, 10], target: 6
    },
    {
      nums: [], target: 0
    }
  ]

  for (const input of inputs) {
    const result = searchRange(input.nums, input.target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
