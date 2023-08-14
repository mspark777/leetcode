import '@total-typescript/ts-reset'

function goright (matrix: number[][], row: number, left: number, right: number, result: number[]): void {
  for (let col = left; col <= right; col += 1) {
    const cell = matrix.at(row)?.at(col) as number
    result.push(cell)
  }
}

function godown (matrix: number[][], col: number, top: number, bottom: number, result: number[]): void {
  for (let row = top; row <= bottom; row += 1) {
    const cell = matrix.at(row)?.at(col) as number
    result.push(cell)
  }
}

function goleft (matrix: number[][], row: number, left: number, right: number, result: number[]): void {
  for (let col = right; col >= left; col -= 1) {
    const cell = matrix.at(row)?.at(col) as number
    result.push(cell)
  }
}

function goup (matrix: number[][], col: number, top: number, bottom: number, result: number[]): void {
  for (let row = bottom; row >= top; row -= 1) {
    const cell = matrix.at(row)?.at(col) as number
    result.push(cell)
  }
}

enum Direction {
  LEFT,
  RIGHT,
  UP,
  DOWN
}

function spiralOrder (matrix: number[][]): number[] {
  const rowCount = matrix.length
  const colCount = matrix[0]?.length as number
  let left = 0
  let right = colCount - 1
  let top = 0
  let bottom = rowCount - 1
  let dir = Direction.RIGHT
  const result: number[] = []

  while ((left <= right) && (top <= bottom)) {
    if (dir === Direction.RIGHT) {
      goright(matrix, top, left, right, result)
      top += 1
      dir = Direction.DOWN
    } else if (dir === Direction.DOWN) {
      godown(matrix, right, top, bottom, result)
      right -= 1
      dir = Direction.LEFT
    } else if (dir === Direction.LEFT) {
      goleft(matrix, bottom, left, right, result)
      bottom -= 1
      dir = Direction.UP
    } else {
      goup(matrix, left, top, bottom, result)
      left += 1
      dir = Direction.RIGHT
    }
  }

  return result
}

function main (): void {
  const inputs: number[][][] = [
    [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
    [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]
  ]

  for (const matrix of inputs) {
    const result = spiralOrder(matrix)
    console.log(result)
  }
}
main()
