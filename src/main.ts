import '@total-typescript/ts-reset'

const QUEEN = 'Q'
const EMPTY = '.'

function isSafe (row: number, col: number, board: string[][]): boolean {
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

function solve (col: number, board: string[][]): number {
  if (col >= board.length) {
    return 1
  }

  let count = 0
  for (let r = 0; r < board.length; r += 1) {
    if (isSafe(r, col, board)) {
      const row = board[r] as string[]
      row[col] = QUEEN
      count += solve(col + 1, board)
      row[col] = EMPTY
    }
  }

  return count
}

function totalNQueens (n: number): number {
  const board = new Array<string[]>(n)
  for (let i = 0; i < n; i += 1) {
    board[i] = new Array<string>(n).fill(EMPTY)
  }

  return solve(0, board)
}

function main (): void {
  const inputs: number[] = [
    4, 1
  ]

  for (const n of inputs) {
    const result = totalNQueens(n)
    console.log(result)
  }
}
main()
