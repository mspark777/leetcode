/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

const QUEEN = 'Q'
const EMPTY = '.'

/**
  * @param {number} row
  * @param {number} col
  * @param {string[][]} board
  * @returns {boolean}
  */
function isSafe (row, col, board) {
  for (let r = row, c = col; (r >= 0) && (c >= 0); r -= 1, c -= 1) {
    if (board.at(r)?.at(c) === QUEEN) {
      return false
    }
  }

  for (let c = col; c >= 0; c -= 1) {
    if (board.at(row)?.at(c) === QUEEN) {
      return false
    }
  }

  for (let r = row, c = col; (r < board.length) && (c >= 0); r += 1, c -= 1) {
    if (board.at(r)?.at(c) === QUEEN) {
      return false
    }
  }

  return true
}

/**
  * @param {number} col
  * @param {string[][]} board
  * @param {string[][]} result
  * @returns {undefined}
  */
function solve (col, board, result) {
  if (col >= board.length) {
    result.push(board.map(b => b.join('')))
    return
  }

  for (let r = 0; r < board.length; r += 1) {
    if (isSafe(r, col, board)) {
      board[r][col] = QUEEN
      solve(col + 1, board, result)
      board[r][col] = EMPTY
    }
  }
}

/**
  * @param {number} n
  * @returns {string[][]}
  */
function solveNQueens (n) {
  /** @type {string[][]} */
  const result = []
  /** @type {string[][]} */
  const board = new Array(n)
  for (let i = 0; i < n; i += 1) {
    board[i] = new Array(n).fill(EMPTY)
  }

  solve(0, board, result)
  return result
}

function main () {
  const inputs = [
    4, 1
  ]

  for (const n of inputs) {
    const result = solveNQueens(n)
    console.log(result)
  }
}
main()
