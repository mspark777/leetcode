import { candy } from './solution'

interface Input {
  readonly ratings: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      ratings: [1, 0, 2]
    },
    {
      ratings: [1, 2, 2]
    },
    {
      ratings: [1, 2, 87, 87, 87, 2, 1]
    },
    {
      ratings: [1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1, 3, 2, 1, 1, 2, 3, 4]
    }
  ]

  for (const input of inputs) {
    const result = candy(input.ratings)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
