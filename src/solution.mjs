export function searchMatrix (matrix, target) {
  let row = matrix.length - 1
  let col = 0
  const countcol = matrix[0].length

  while ((row >= 0) && (col < countcol)) {
    const n = matrix[row][col]
    if (target > n) {
      col += 1
    } else if (target < n) {
      row -= 1
    } else {
      return true
    }
  }

  return false
}
