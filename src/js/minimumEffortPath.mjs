/**
 * @param {number[][]} heights
 * @return {number}
 */
export const minimumEffortPath = function (heights) {
  if (heights.length === 1 && heights[0].length === 1) {
    return 0
  }

  const calcEffort = (p0, p1) => {
    const height0 = heights[p0.row][p0.col]
    const height1 = heights[p1.row][p1.col]
    return Math.abs(height0 - height1)
  }

  const efforts = []
  const visited = []
  for (const cols of heights) {
    efforts.push(cols.map(_ => Number.MAX_SAFE_INTEGER))
    visited.push(cols.map(_ => false))
  }
  efforts[0][0] = 0

  const inRange = (i, max) => (i >= 0) && (i < max)
  const inHeights = pos => inRange(pos.row, heights.length) && inRange(pos.col, heights[pos.row].length)
  const heap = [{ row: 0, col: 0, height: 0 }]
  const visit = (from, to) => {
    if (!inHeights(to)) {
      return
    } else if (visited[to.row][to.col]) {
      return
    }

    const effort = calcEffort(from, to)
    const max = Math.max(efforts[from.row][from.col], effort)
    if (efforts[to.row][to.col] > max) {
      efforts[to.row][to.col] = max
      to.effort = max
      heap.push(to)
    }
  }

  const endRow = heights.length - 1
  const endCol = heights[endRow].length - 1
  const dirs = [0, 1, 0, -1, 0]
  while (heap.length > 0) {
    const pos = heap.pop()
    if (pos.row === endRow && pos.col === endCol) {
      return pos.effort
    }
    visited[pos.row][pos.col] = true

    for (let i = 0; i < 4; i += 1) {
      visit(pos, {
        row: pos.row + dirs[i],
        col: pos.col + dirs[i + 1]
      })
    }

    heap.sort((a, b) => b.effort - a.effort)
  }
}
