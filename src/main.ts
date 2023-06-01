import '@total-typescript/ts-reset'

interface Node {
  readonly x: number
  readonly y: number
  readonly length: number
}

function outRange (x: number, y: number, dx: number, dy: number): boolean {
  return (x < 0) || (y < 0) || (x > dx) || (y > dy)
}

function reached (x: number, y: number, dx: number, dy: number): boolean {
  return (x === dx) && (y === dy)
}

function shortestPathBinaryMatrix (grid: number[][]): number {
  const VISIT = 1
  const NOT_FOUND = -1

  const dx = grid[0].length - 1
  const dy = grid.length - 1
  if (grid[dy][dx] === VISIT) {
    return NOT_FOUND
  }

  const nexts = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1]
  ]

  let result = Number.MAX_SAFE_INTEGER
  const queue: Node[] = [{ x: 0, y: 0, length: 1 }]
  for (let node = queue.shift(); node != null; node = queue.shift()) {
    if (reached(node.x, node.y, dx, dy)) {
      result = Math.min(result, node.length)
      continue
    } else if (grid[node.y][node.x] === VISIT) {
      continue
    }

    const length = node.length + 1
    if (length >= result) {
      continue
    }

    grid[node.y][node.x] = VISIT

    for (const [nx, ny] of nexts) {
      const x = node.x + nx
      const y = node.y + ny
      if (!outRange(x, y, dx, dy)) {
        queue.push({ x, y, length })
      }
    }
  }

  return result < Number.MAX_SAFE_INTEGER ? result : NOT_FOUND
}

function main (): void {
  const inputs = [
    [[0, 1], [1, 0]],
    [[0, 0, 0], [1, 1, 0], [1, 1, 0]],
    [[1, 0, 0], [1, 1, 0], [1, 1, 0]],
    [[0, 0, 0], [1, 1, 0], [1, 1, 1]]
  ]

  for (const grid of inputs) {
    const result = shortestPathBinaryMatrix(grid)
    console.log(result)
  }
}
main()
