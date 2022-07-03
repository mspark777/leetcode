import { wiggleMaxLength } from './solution'

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [1, 7, 4, 9, 2, 5]
    },
    {
      nums: [1, 17, 5, 10, 13, 15, 10, 5, 16, 8]
    },
    {
      nums: [1, 2, 3, 4, 5, 6, 7, 8, 9]
    }
  ]

  for (const input of inputs) {
    const result = wiggleMaxLength(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
