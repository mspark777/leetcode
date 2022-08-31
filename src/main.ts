function inLand (row: number, col: number, rowCount: number, colCount: number): boolean {
  return (row >= 0) && (row < rowCount) && (col >= 0) && (col < colCount)
}

function visit (heights: number[][], queue: number[][]): boolean[][] {
  const rowCount = heights.length
  const colCount = heights[0].length
  const visiteds = new Array<boolean[]>(rowCount)
  for (let i = 0; i < heights.length; i += 1) {
    visiteds[i] = new Array<boolean>(colCount).fill(false)
  }

  const dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]]
  for (let top = queue.shift(); top != null; top = queue.shift()) {
    const [row, col] = top
    visiteds[row][col] = true

    for (const [r, c] of dirs) {
      const nextRow = row + r
      const nextCol = col + c

      if (!inLand(nextRow, nextCol, rowCount, colCount)) {
        continue
      }

      const height = heights[row][col]
      const nextHeight = heights[nextRow][nextCol]
      const visited = visiteds[nextRow][nextCol]
      if (visited || (nextHeight < height)) {
        continue
      }

      queue.push([nextRow, nextCol])
      visiteds[nextRow][nextCol] = true
    }
  }

  return visiteds
}

function pacificAtlantic (heights: number[][]): number[][] {
  const rowCount = heights.length
  const colCount = heights[0].length
  const pacifics: number[][] = []
  const atlantics: number[][] = []

  for (let r = 0; r < rowCount; r += 1) {
    pacifics.push([r, 0])
    atlantics.push([r, colCount - 1])
  }

  for (let c = 0; c < colCount; c += 1) {
    pacifics.push([0, c])
    atlantics.push([rowCount - 1, c])
  }

  const pacificVisiteds = visit(heights, pacifics)
  const atlanticVisiteds = visit(heights, atlantics)
  const result: number[][] = []
  for (let r = 0; r < rowCount; r += 1) {
    for (let c = 0; c < colCount; c += 1) {
      if (pacificVisiteds[r][c] && atlanticVisiteds[r][c]) {
        result.push([r, c])
      }
    }
  }

  return result
}

interface Input {
  readonly heights: number[][]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      heights: [
        [1, 2, 2, 3, 5],
        [3, 2, 3, 4, 4],
        [2, 4, 5, 3, 1],
        [6, 7, 1, 4, 5],
        [5, 1, 1, 2, 4]
      ]
    },
    {
      heights: [
        [1]
      ]
    }
  ]

  for (const { heights } of inputs) {
    const result = pacificAtlantic(heights)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
