/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[][]} matrix
  * @param {number} row
  * @param {number} left
  * @param {number} right
  * @param {number[]} result
  * @returns {undefined}
  */
function goright (matrix, row, left, right, result) {
  for (let col = left; col <= right; col += 1) {
    const cell = matrix[row][col]
    result.push(cell)
  }
}

/**
  * @param {number[][]} matrix
  * @param {number} col
  * @param {number} top
  * @param {number} bottom
  * @param {number[]} result
  * @returns {undefined}
  */
function godown (matrix, col, top, bottom, result) {
  for (let row = top; row <= bottom; row += 1) {
    const cell = matrix[row][col]
    result.push(cell)
  }
}

/**
  * @param {number[][]} matrix
  * @param {number} row
  * @param {number} left
  * @param {number} right
  * @param {number[]} result
  * @returns {undefined}
  */
function goleft (matrix, row, left, right, result) {
  for (let col = right; col >= left; col -= 1) {
    const cell = matrix[row][col]
    result.push(cell)
  }
}

/**
  * @param {number[][]} matrix
  * @param {number} col
  * @param {number} top
  * @param {number} bottom
  * @param {number[]} result
  * @returns {undefined}
  */
function goup (matrix, col, top, bottom, result) {
  for (let row = bottom; row >= top; row -= 1) {
    const cell = matrix[row][col]
    result.push(cell)
  }
}

const Direction = {
  LEFT: 0,
  RIGHT: 1,
  UP: 2,
  DOWN: 3
}

/**
  * @param {number[][]} matrix
  * @returns {number[]}
  */
function spiralOrder (matrix) {
  const rowCount = matrix.length
  const colCount = matrix[0].length
  let left = 0
  let right = colCount - 1
  let top = 0
  let bottom = rowCount - 1
  let dir = Direction.RIGHT
  /** @type {number[]} */
  const result = []

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

function main () {
  const inputs = [
    [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
    [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]
  ]

  for (const matrix of inputs) {
    const result = spiralOrder(matrix)
    console.log(result)
  }
}
main()
