export class NumMatrix {
  readonly matrix: number[][]
  constructor (matrix: number[][]) {
    const rowLen = matrix.length
    const colLen = matrix[0].length
    const dp = new Array<number[]>(rowLen + 1)
    for (let i = 0; i <= rowLen; i += 1) {
      const row = new Array<number>(colLen + 1).fill(0)
      dp[i] = row
    }

    for (let i = 0; i < rowLen; i += 1) {
      for (let j = 0; j < colLen; j += 1) {
        dp[i + 1][j + 1] = dp[i + 1][j] + dp[i][j + 1] + matrix[i][j] - dp[i][j]
      }
    }

    this.matrix = dp
  }

  sumRegion (row1: number, col1: number, row2: number, col2: number): number {
    const matrix = this.matrix
    return matrix[row2 + 1][col2 + 1] - matrix[row1][col2 + 1] - matrix[row2 + 1][col1] + matrix[row1][col1]
  }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * var obj = new NumMatrix(matrix)
 * var param_1 = obj.sumRegion(row1,col1,row2,col2)
 */
