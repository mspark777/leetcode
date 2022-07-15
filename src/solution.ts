export function maxAreaOfIsland (grid: number[][]): number {
  const rowCount = grid.length
  const colCount = grid[0].length
  const seen: boolean[][] = new Array(rowCount)
  for (let i = 0; i < rowCount; i += 1) {
    seen[i] = new Array<boolean>(colCount).fill(false)
  }

  const dr = [1, -1, 0, 0]
  const dc = [0, 0, 1, -1]
  const outRange = (n: number, end: number) => (n < 0) || (n >= end)

  let result = 0
  const stack: number[][] = []
  for (let r = 0; r < rowCount; r += 1) {
    for (let c = 0; c < colCount; c += 1) {
      if ((grid[r][c] === 0) || seen[r][c]) {
        continue
      }

      let shape = 0
      stack.push([r, c])
      seen[r][c] = true
      while (stack.length > 0) {
        const [row, col] = stack.pop() as number[]
        shape += 1
        for (let i = 0; i < 4; i += 1) {
          const nr = row + dr[i]
          const nc = col + dc[i]
          if (outRange(nr, rowCount)) {
            continue
          } else if (outRange(nc, colCount)) {
            continue
          } else if (grid[nr][nc] === 0) {
            continue
          } else if (seen[nr][nc]) {
            continue
          } else {
            stack.push([nr, nc])
            seen[nr][nc] = true
          }
        }
        result = Math.max(result, shape)
      }
    }
  }

  return result
}
