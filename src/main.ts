import { containsDuplicate } from './solution'

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [1, 2, 3, 1]
    },
    {
      nums: [1, 2, 3, 4]
    },
    {
      nums: [1, 1, 1, 3, 3, 4, 3, 2, 4, 2]
    }
  ]

  for (const input of inputs) {
    const result = containsDuplicate(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
