/**
 * @param {number[][]} matrix
 * @return {void}
 */
function transpose (matrix) {
  for (let i = 0; i < matrix.length; i += 1) {
    for (let j = i + 1; j < matrix.length; j += 1) {
      const temp = matrix[i][j]
      matrix[i][j] = matrix[j][i]
      matrix[j][i] = temp
    }
  }
}

/**
 * @param {number[][]} matrix
 * @return {void}
 */
function reverse (matrix) {
  for (let i = 0; i < matrix.length; i += 1) {
    let j = 0
    let k = matrix.length - 1
    while (j < k) {
      const temp = matrix[i][j]
      matrix[i][j] = matrix[i][k]
      matrix[i][k] = temp
      j += 1
      k -= 1
    }
  }
}

/**
 * @param {number[][]} matrix
 * @return {void} Do not return anything, modify matrix in-place instead.
 */
function rotate (matrix) {
  transpose(matrix)
  reverse(matrix)
}

async function main () {
  const inputs = [
    {
      matrix: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    },
    {
      matrix: [[5, 1, 9, 11], [2, 4, 8, 10], [13, 3, 6, 7], [15, 14, 12, 16]]
    }
  ]

  for (const { matrix } of inputs) {
    rotate(matrix)
    console.log(matrix)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
