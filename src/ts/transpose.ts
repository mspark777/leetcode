export function transpose (matrix: number[][]): number[][] {
  const rowLen = matrix.length
  const colLen = matrix[0].length
  const transposed = new Array<number[]>(colLen)
  for (let i = 0; i < colLen; i += 1) {
    const row = new Array<number>(rowLen)
    for (let j = 0; j < rowLen; j += 1) {
      row[j] = matrix[j][i]
    }
    transposed[i] = row
  }

  return transposed
}
