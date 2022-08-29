const LAND = '1'
const WATER = '0'

/**
 * @param {string[][]} grid
 * @param {number} row
 * @param {number} col
 * @param {number} row
 * @param {number} rowCount
 * @param {number} colCount
 * @return {number}
 */
function clearLand (grid, row, col, rowCount, colCount) {
  const stack = [[row, col]]

  while (stack.length > 0) {
    const [r, c] = stack.pop()

    if (r < 0) {
      continue
    } else if (r >= rowCount) {
      continue
    } else if (c < 0) {
      continue
    } else if (c >= colCount) {
      continue
    } else if (grid[r][c] === WATER) {
      continue
    }

    grid[r][c] = WATER
    stack.push([r - 1, c])
    stack.push([r + 1, c])
    stack.push([r, c - 1])
    stack.push([r, c + 1])
  }
}

/**
 * @param {string[][]} grid
 * @return {number}
 */
function numIslands (grid) {
  const rowCount = grid.length
  const colCount = grid[0].length

  let result = 0
  for (let r = 0; r < rowCount; r += 1) {
    for (let c = 0; c < colCount; c += 1) {
      if (grid[r][c] === LAND) {
        result += 1
        clearLand(grid, r, c, rowCount, colCount)
      }
    }
  }

  return result
}

async function main () {
  const inputs = [
    {
      grid: [
        ['1', '1', '1', '1', '0'],
        ['1', '1', '0', '1', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '0', '0', '0']
      ]
    },
    {
      grid: [
        ['1', '1', '0', '0', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '1', '0', '0'],
        ['0', '0', '0', '1', '1']
      ]
    }
  ]

  for (const { grid } of inputs) {
    const result = numIslands(grid)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
