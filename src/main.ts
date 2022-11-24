function dfs (seens: Set<string>, board: string[][], row: number, col: number, length: number, word: string): boolean {
  if (length === word.length) {
    return true
  }

  if ((row < 0) || (col < 0)) {
    return false
  } else if ((row >= board.length) || (col >= board[0].length)) {
    return false
  }

  const seen = `${row}|${col}`
  if (seens.has(seen)) {
    return false
  } else if (board[row][col] !== word[length]) {
    return false
  }

  seens.add(seen)
  const found = dfs(seens, board, row + 1, col, length + 1, word) ||
    dfs(seens, board, row - 1, col, length + 1, word) ||
    dfs(seens, board, row, col + 1, length + 1, word) ||
    dfs(seens, board, row, col - 1, length + 1, word)

  seens.delete(seen)

  return found
}

function exist (board: string[][], word: string): boolean {
  for (let r = 0; r < board.length; r += 1) {
    for (let c = 0; c < board[r].length; c += 1) {
      if (board[r][c] === word[0]) {
        const seens = new Set<string>()
        if (dfs(seens, board, r, c, 0, word)) {
          return true
        }
      }
    }
  }

  return false
}

interface Input {
  readonly board: string[][]
  readonly word: string
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      board: [['A', 'B', 'C', 'E'], ['S', 'F', 'C', 'S'], ['A', 'D', 'E', 'E']],
      word: 'ABCCED'
    },
    {
      board: [['A', 'B', 'C', 'E'], ['S', 'F', 'C', 'S'], ['A', 'D', 'E', 'E']],
      word: 'SEE'
    },
    {
      board: [['A', 'B', 'C', 'E'], ['S', 'F', 'C', 'S'], ['A', 'D', 'E', 'E']],
      word: 'ABCB'
    },
    {
      board: [['A', 'A', 'A', 'A', 'A', 'A'], ['A', 'A', 'A', 'A', 'A', 'A'], ['A', 'A', 'A', 'A', 'A', 'A'], ['A', 'A', 'A', 'A', 'A', 'A'], ['A', 'A', 'A', 'A', 'A', 'A'], ['A', 'A', 'A', 'A', 'A', 'A']],
      word: 'AAAAAAAAAAAABAA'
    },
    {
      board: [['a']],
      word: 'a'
    }
  ]

  for (const { board, word } of inputs) {
    const result = exist(board, word)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
