export function numSubmatrixSumTarget (matrix: number[][], target: number): number {
  let result = 0
  const m = matrix.length
  const n = matrix[0].length

  for (let i = 0; i < m; i += 1) {
    for (let j = 1; j < n; j += 1) {
      matrix[i][j] += matrix[i][j - 1]
    }
  }

  const counter = new Map<number, number>()
  for (let i = 0; i < n; i += 1) {
    for (let j = i; j < n; j += 1) {
      counter.clear()
      counter.set(0, 1)
      let cur = 0
      for (let k = 0; k < m; k += 1) {
        cur += matrix[k][j]
        if (i > 0) {
          cur -= matrix[k][i - 1]
        }

        result += counter.get(cur - target) ?? 0
        counter.set(cur, (counter.get(cur) ?? 0) + 1)
      }
    }
  }

  return result
}
