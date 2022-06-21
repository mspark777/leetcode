/**
 * @param {number[][]} matrix
 * @return {number}
 */
export const longestIncreasingPath = function (matrix) {
  const yLength = matrix.length
  const xLength = matrix[0].length
  const maxX = xLength - 1
  const maxY = yLength - 1
  const memos = new Array(yLength)
  const cellInMatrix = (x, y) =>
    (x >= 0) &&
    (y >= 0) &&
    (x <= maxX) &&
    (y <= maxY)

  for (let i = 0; i < yLength; i += 1) {
    memos[i] = new Array(xLength)
  }

  const steps = [[-1, 0], [1, 0], [0, -1], [0, 1]]

  const dfs = (x, y) => {
    let memo = memos[y][x]
    if (memo) {
      return memo
    }
    memo = 0

    const value = matrix[y][x]
    for (const step of steps) {
      const newX = x + step[0]
      const newY = y + step[1]
      if (cellInMatrix(newX, newY)) {
        const next = matrix[newY][newX]
        if (next < value) {
          memo = Math.max(dfs(newX, newY), memo)
        }
      }
    }

    memo += 1
    memos[y][x] = memo
    return memo
  }

  let result = 0
  for (let y = 0; y < yLength; y += 1) {
    for (let x = 0; x < xLength; x += 1) {
      result = Math.max(dfs(x, y), result)
    }
  }

  return result
}
