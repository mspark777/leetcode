/**
 * @param {number[][]} grid
 * @return {number}
 */
export const shortestPathBinaryMatrix = function (grid) {
  const dx = grid[0].length - 1
  const dy = grid.length - 1

  const visits = new Set(['00'])
  const queue = [{
    x: 0,
    y: 0,
    l: 1
  }]

  let result = Number.MAX_SAFE_INTEGER
  const nexts = []
  for (let i = -1; i < 2; i += 1) {
    for (let j = -1; j < 2; j += 1) {
      if ((i !== 0) || (j !== 0)) {
        nexts.push({ x: i, y: j })
      }
    }
  }

  while (queue.length > 0) {
    const front = queue.shift()
    const length = front.l
    if ((front.x < 0) || (front.y < 0)) {
      continue
    } else if ((front.x > dx) || (front.y > dy)) {
      continue
    } else if (grid[front.y][front.x] !== 0) {
      continue
    } else if ((front.x === dx) && (front.y === dy)) {
      result = Math.min(result, length)
      continue
    }

    const nextLength = length + 1
    for (const n of nexts) {
      const x = front.x + n.x
      const y = front.y + n.y
      const key = `${x}${y}`
      if (!visits.has(key)) {
        visits.add(key)
        queue.push({
          x,
          y,
          l: nextLength
        })
      }
    }
  }

  if (result === Number.MAX_SAFE_INTEGER) {
    return -1
  }

  return result
}
