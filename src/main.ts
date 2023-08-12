import '@total-typescript/ts-reset'

function getCell (grid: number[][], r: number, c: number): number {
  const row = grid[r] as number[]
  return row[c] as number
}

function setCell (grid: number[][], r: number, c: number, v: number): void {
  const row = grid[r] as number[]
  row[c] = v
}

function uniquePathsWithObstacles (obstacleGrid: number[][]): number {
  const OBSTACLE = 1
  const rowCount = obstacleGrid.length
  const colCount = obstacleGrid[0]?.length as number
  const countGrid = new Array<number[]>(rowCount)
  for (let r = 0; r < rowCount; r += 1) {
    countGrid[r] = new Array(colCount).fill(0)
  }

  for (let r = 0; r < rowCount; r += 1) {
    for (let c = 0; c < colCount; c += 1) {
      if (getCell(obstacleGrid, r, c) === OBSTACLE) {
        continue
      }

      if ((r + c) === 0) {
        setCell(countGrid, r, c, 1)
      } else if (r === 0) {
        setCell(countGrid, r, c, getCell(countGrid, r, c - 1))
      } else if (c === 0) {
        setCell(countGrid, r, c, getCell(countGrid, r - 1, c))
      } else {
        setCell(countGrid, r, c,
          getCell(countGrid, r - 1, c) +
          getCell(countGrid, r, c - 1)
        )
      }
    }
  }

  return getCell(countGrid, rowCount - 1, colCount - 1)
}

function main (): void {
  const inputs: number[][][] = [
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[0, 1], [0, 0]]
  ]

  for (const input of inputs) {
    const result = uniquePathsWithObstacles(input)
    console.log(result)
  }
}
main()
