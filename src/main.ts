import { makesquare } from './solution'

interface Input {
  readonly matchsticks: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      matchsticks: [1, 1, 2, 2, 2]
    },
    {
      matchsticks: [3, 3, 3, 3, 4]
    },
    {
      matchsticks: [6, 6, 6, 6, 4, 2, 2]
    }
  ]

  for (const input of inputs) {
    const result = makesquare(input.matchsticks)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
