/**
 * @param {number} row
 * @param {number} col
 * @param {number} rowCount
 * @param {number} colCount
 * @returns {boolean}
*/
function inLand (row, col, rowCount, colCount) {
  return (row >= 0) && (row < rowCount) && (col >= 0) && (col < colCount)
}

/**
 * @param {number[][]} heights
 * @param {number[][]} stack
 * @returns {boolean[][]}
*/
function visit (heights, stack) {
  const rowCount = heights.length
  const colCount = heights[0].length
  const visiteds = new Array(rowCount)
  for (let i = 0; i < heights.length; i += 1) {
    visiteds[i] = new Array(colCount).fill(false)
  }

  const dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]]
  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const [row, col] = top
    visiteds[row][col] = true

    for (const [r, c] of dirs) {
      const nextRow = row + r
      const nextCol = col + c

      if (!inLand(nextRow, nextCol, rowCount, colCount)) {
        continue
      }

      const height = heights[row][col]
      const nextHeight = heights[nextRow][nextCol]
      const visited = visiteds[nextRow][nextCol]
      if (visited || (nextHeight < height)) {
        continue
      }

      stack.push([nextRow, nextCol])
      visiteds[nextRow][nextCol] = true
    }
  }

  return visiteds
}

/**
 * @param {number[][]} heights
 * @return {number[][]}
 */
function pacificAtlantic (heights) {
  const rowCount = heights.length
  const colCount = heights[0].length
  const pacifics = []
  const atlantics = []

  for (let r = 0; r < rowCount; r += 1) {
    pacifics.push([r, 0])
    atlantics.push([r, colCount - 1])
  }

  for (let c = 0; c < colCount; c += 1) {
    pacifics.push([0, c])
    atlantics.push([rowCount - 1, c])
  }

  const pacificVisiteds = visit(heights, pacifics)
  const atlanticVisiteds = visit(heights, atlantics)
  const result = []
  for (let r = 0; r < rowCount; r += 1) {
    for (let c = 0; c < colCount; c += 1) {
      if (pacificVisiteds[r][c] && atlanticVisiteds[r][c]) {
        result.push([r, c])
      }
    }
  }

  return result
}

async function main () {
  const inputs = [
    {
      heights: [
        [1, 2, 2, 3, 5],
        [3, 2, 3, 4, 4],
        [2, 4, 5, 3, 1],
        [6, 7, 1, 4, 5],
        [5, 1, 1, 2, 4]
      ]
    },
    {
      heights: [
        [1]
      ]
    }
  ]

  for (const { heights } of inputs) {
    const result = pacificAtlantic(heights)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
