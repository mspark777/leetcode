function nearestExit (maze: string[][], entrance: number[]): number {
  const WALL = '+'
  const rowCount = maze.length
  const colCount = maze[0].length
  const lastRow = rowCount - 1
  const lastCol = colCount - 1
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]]

  const queue: number[][] = [[...entrance, 0]]
  maze[entrance[0]][entrance[1]] = WALL
  for (let front = queue.shift(); front != null; front = queue.shift()) {
    const [row, col, steps] = front

    const nextSteps = steps + 1
    for (const [r, c] of dirs) {
      const nextRow = row + r
      const nextCol = col + c
      if (nextRow < 0) {
        continue
      } else if (nextRow >= rowCount) {
        continue
      } else if (nextCol < 0) {
        continue
      } else if (nextCol >= colCount) {
        continue
      } else if (maze[nextRow][nextCol] === WALL) {
        continue
      }

      if (nextRow === 0) {
        return nextSteps
      } else if (nextRow === lastRow) {
        return nextSteps
      } else if (nextCol === 0) {
        return nextSteps
      } else if (nextCol === lastCol) {
        return nextSteps
      }

      maze[nextRow][nextCol] = WALL
      queue.push([nextRow, nextCol, nextSteps])
    }
  }

  return -1
}

interface Input {
  readonly maze: string[][]
  readonly entrance: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      maze: [['+', '+', '.', '+'], ['.', '.', '.', '+'], ['+', '+', '+', '.']],
      entrance: [1, 2]
    },
    {
      maze: [['+', '+', '+'], ['.', '.', '.'], ['+', '+', '+']],
      entrance: [1, 0]
    },
    {
      maze: [['.', '+']],
      entrance: [0, 0]
    }
  ]

  for (const { maze, entrance } of inputs) {
    const result = nearestExit(maze, entrance)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
