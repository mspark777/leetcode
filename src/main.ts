import { searchMatrix } from './solution'

interface Input {
  readonly matrix: number[][]
  readonly target: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      matrix: [[1, 4, 7, 11, 15], [2, 5, 8, 12, 19], [3, 6, 9, 16, 22], [10, 13, 14, 17, 24], [18, 21, 23, 26, 30]],
      target: 5
    },
    {
      matrix: [[1, 4, 7, 11, 15], [2, 5, 8, 12, 19], [3, 6, 9, 16, 22], [10, 13, 14, 17, 24], [18, 21, 23, 26, 30]],
      target: 20
    }
  ]

  for (const input of inputs) {
    const result = searchMatrix(input.matrix, input.target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
