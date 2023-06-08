import '@total-typescript/ts-reset'

function countNegatives (grid: number[][]): number {
  let result = 0
  const colCount = grid[0].length
  let cur = colCount - 1

  for (const row of grid) {
    while (cur >= 0) {
      if (row[cur] < 0) {
        cur -= 1
      } else {
        break
      }
    }

    result += colCount - (cur + 1)
  }

  return result
}

function main (): void {
  const inputs = [
    [[4, 3, 2, -1], [3, 2, 1, -1], [1, 1, -1, -2], [-1, -1, -2, -3]],
    [[3, 2], [1, 0]]
  ]

  for (const grid of inputs) {
    const result = countNegatives(grid)
    console.log(result)
  }
}
main()
