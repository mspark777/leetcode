export function minimumTotal (triangle: number[][]): number {
  for (let i = triangle.length - 2; i >= 0; i -= 1) {
    for (let j = 0; j < triangle[i].length; j += 1) {
      const n0 = triangle[i + 1][j]
      const n1 = triangle[i + 1][j + 1]
      triangle[i][j] += Math.min(n0, n1)
    }
  }

  return triangle[0][0]
}
