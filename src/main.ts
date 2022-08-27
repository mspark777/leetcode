function maxSumSubmatrix (matrix: number[][], k: number): number {
  const rowCount = matrix.length
  const colCount = matrix[0].length
  let maxSum = Number.MIN_SAFE_INTEGER

  for (let i0 = 0; i0 < colCount; i0 += 1) {
    const sums = new Array<number>(rowCount).fill(0)
    for (let i1 = i0; i1 < colCount; i1 += 1) {
      for (let i2 = 0; i2 < rowCount; i2 += 1) {
        sums[i2] += matrix[i2][i1]
      }

      for (let i2 = 0; i2 < rowCount; i2 += 1) {
        let sum = 0
        for (let i3 = i2; i3 < rowCount; i3 += 1) {
          sum += sums[i3]
          if ((sum > maxSum) && (sum <= k)) {
            maxSum = sum
          }
        }
      }
    }
  }

  return maxSum
}

interface Input {
  readonly matrix: number[][]
  readonly k: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      matrix: [[1, 0, 1], [0, -2, 3]],
      k: 2
    },
    {
      matrix: [[2, 2, -1]],
      k: 3
    }
  ]

  for (const { matrix, k } of inputs) {
    const result = maxSumSubmatrix(matrix, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
