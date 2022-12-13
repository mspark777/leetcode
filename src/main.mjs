/**
 * @param {number[][]} matrix
 * @returns {number}
*/
function minFallingPathSum (matrix) {
  const rowCount = matrix.length
  const colCount = matrix[0].length
  const lastRowIdx = rowCount - 1
  const lastColIdx = colCount - 1

  const dp = new Array(rowCount)
  for (let i = 0; i < rowCount; i += 1) {
    dp[i] = new Array(colCount).fill(0)
  }

  for (let i = 0; i < colCount; i += 1) {
    dp[lastRowIdx][i] = matrix[lastRowIdx][i]
  }

  for (let i = rowCount - 2; i >= 0; i -= 1) {
    for (let j = 0; j < colCount; j += 1) {
      const next = i + 1
      let min = Number.MAX_SAFE_INTEGER
      if (j < lastColIdx) {
        min = Math.min(dp[next][j + 1], min)
      }

      if (j > 0) {
        min = Math.min(dp[next][j - 1], min)
      }

      min = Math.min(dp[next][j], min)
      dp[i][j] = matrix[i][j] + min
    }
  }

  let min = Number.MAX_SAFE_INTEGER
  for (let i = 0; i < colCount; i += 1) {
    min = Math.min(dp[0][i], min)
  }

  return min
}

async function main () {
  const inputs = [
    [[2, 1, 3], [6, 5, 4], [7, 8, 9]],
    [[-19, 57], [-40, -5]]
  ]

  for (const matrix of inputs) {
    const result = minFallingPathSum(matrix)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
