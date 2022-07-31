import { NumArray } from './solution'

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [1, 3, 5]
    }
  ]

  for (const input of inputs) {
    const narr = new NumArray(input.nums)
    console.log(narr.sumRange(0, 2))
    narr.update(1, 2)
    console.log(narr.sumRange(0, 2))
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
