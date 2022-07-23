import { countSmaller } from './solution'

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [5, 2, 6, 1]
    },
    {
      nums: [-1]
    },
    {
      nums: [-1, -1]
    }
  ]

  for (const input of inputs) {
    const result = countSmaller(input.nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
