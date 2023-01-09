const MAX = 9
const ROW = 9
const COL = 9
const EMPTY = '.'

/**
 * @param {number} row
 * @param {number} col
 * @param {string} cell
 * @param {string[][]} board
 * @returns {boolean}
 */
function isValid (row, col, cell, board) {
  for (let i = 0; i < MAX; i += 1) {
    if (board[row][i] === cell) {
      return false
    }

    if (board[i][col] === cell) {
      return false
    }

    const r = 3 * Math.trunc(row / 3) + Math.trunc(i / 3)
    const c = 3 * Math.trunc(col / 3) + (i % 3)
    if (board[r][c] === cell) {
      return false
    }
  }

  return true
}

/**
 * @param {string[][]} board
 * @returns {boolean}
 */
function solve (board) {
  for (let r = 0; r < ROW; r += 1) {
    for (let c = 0; c < COL; c += 1) {
      if (board[r][c] === EMPTY) {
        for (let i = 1; i <= MAX; i += 1) {
          const cell = i.toString()
          if (isValid(r, c, cell, board)) {
            board[r][c] = cell
            if (solve(board)) {
              return true
            }
            board[r][c] = EMPTY
          }
        }
        return false
      }
    }
  }
  return true
}

/**
 * Do not return anything, modify board in-place instead.
 * @param {string[][]} board
 * @returns {undefined}
 */
function solveSudoku (board) {
  solve(board)
}

async function main () {
  const inputs = [
    [['5', '3', '.', '.', '7', '.', '.', '.', '.'], ['6', '.', '.', '1', '9', '5', '.', '.', '.'], ['.', '9', '8', '.', '.', '.', '.', '6', '.'], ['8', '.', '.', '.', '6', '.', '.', '.', '3'], ['4', '.', '.', '8', '.', '3', '.', '.', '1'], ['7', '.', '.', '.', '2', '.', '.', '.', '6'], ['.', '6', '.', '.', '.', '.', '2', '8', '.'], ['.', '.', '.', '4', '1', '9', '.', '.', '5'], ['.', '.', '.', '.', '8', '.', '.', '7', '9']]
  ]

  for (const board of inputs) {
    solveSudoku(board)
    console.log(board)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
