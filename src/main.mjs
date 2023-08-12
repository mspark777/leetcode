/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[][]} grid
  * @param {number} r
  * @param {number} c
  * @returns {number}
  */
function getCell (grid, r, c) {
  const row = grid[r]
  return row[c]
}

/**
  * @param {number[][]} grid
  * @param {number} r
  * @param {number} c
  * @param {number} v
  * @returns {number}
  */
function setCell (grid, r, c, v) {
  const row = grid[r]
  row[c] = v
}

/**
  * @param {number[][]} obstacleGrid
  * @returns {number}
  */
function uniquePathsWithObstacles (obstacleGrid) {
  const OBSTACLE = 1
  const rowCount = obstacleGrid.length
  const colCount = obstacleGrid[0]?.length
  /** @type{number[][]} */
  const countGrid = new Array(rowCount)
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

function main () {
  const inputs = [
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[0, 1], [0, 0]]
  ]

  for (const input of inputs) {
    const result = uniquePathsWithObstacles(input)
    console.log(result)
  }
}
main()
