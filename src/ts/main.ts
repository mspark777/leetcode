function minPathSum (grid: number[][]): number {
  const row = grid.length
  const col = grid[0].length
  const lastRow = row - 1
  const lastCol = col - 1
  const MAX = Number.MAX_SAFE_INTEGER

  for (let r = lastRow; r >= 0; r -= 1) {
    for (let c = lastCol; c >= 0; c -= 1) {
      if ((r === lastRow) && (c === lastCol)) {
        continue
      }

      const rightMin = c >= lastCol ? MAX : grid[r][c + 1]
      const downMin = r >= lastRow ? MAX : grid[r + 1][c]
      grid[r][c] += Math.min(rightMin, downMin)
    }
  }

  return grid[0][0]
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1, 3, 1], [1, 5, 1], [4, 2, 1]],
    [[1, 2, 3], [4, 5, 6]]
  ]

  for (const grid of inputs) {
    const result = minPathSum(grid)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
