function isToeplitzMatrix (matrix: number[][]): boolean {
  for (let r = 1; r < matrix.length; r += 1) {
    for (let c = 1; c < matrix[r].length; c += 1) {
      if (matrix[r - 1][c - 1] !== matrix[r][c]) {
        return false
      }
    }
  }

  return true
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]],
    [[1, 2], [2, 2]]
  ]

  for (const matrix of inputs) {
    const result = isToeplitzMatrix(matrix)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
