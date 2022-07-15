import { maxAreaOfIsland } from './solution'

interface Input {
  readonly grid: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      grid: [
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
      ]
    },
    {
      grid: [[0, 0, 0, 0, 0, 0, 0, 0]]
    },
    {
      grid: [[1, 1, 0, 0, 0], [1, 1, 0, 0, 0], [0, 0, 0, 1, 1], [0, 0, 0, 1, 1]]
    }
  ]

  for (const input of inputs) {
    const result = maxAreaOfIsland(input.grid)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
