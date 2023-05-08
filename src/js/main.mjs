/**
  * @param {number[][]} mat
  * @returns {number}
  */
function diagonalSum (mat) {
  let result = 0
  for (let left = 0; left < mat.length; left += 1) {
    const right = mat.length - (left + 1)
    result += mat[left][left] + mat[right][left]
  }

  if ((mat.length % 2) === 1) {
    const m = Math.trunc(mat.length / 2)
    result -= mat[m][m]
  }

  return result
}

async function main () {
  const inputs = [
    [
      [1, 2, 3],
      [4, 5, 6],
      [7, 8, 9]
    ],
    [
      [1, 1, 1, 1],
      [1, 1, 1, 1],
      [1, 1, 1, 1],
      [1, 1, 1, 1]
    ],
    [
      [5]
    ]
  ]

  for (const mat of inputs) {
    const result = diagonalSum(mat)
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
