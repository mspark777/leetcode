function isValidSudoku (board: string[][]): boolean {
  const seens = new Set<string>()
  for (let r = 0; r < 9; r += 1) {
    for (let c = 0; c < 9; c += 1) {
      const n = board[r][c]
      if (n === '.') {
        continue
      }

      const ns = `(${n})`
      const row = `${ns}${r}`
      const col = `${c}${ns}`
      const cross = `${BigInt(r) / 3n}${ns}${BigInt(c) / 3n}`
      if (seens.has(row) || seens.has(col) || seens.has(cross)) {
        return false
      }

      seens
        .add(row)
        .add(col)
        .add(cross)
    }
  }

  return true
}

async function main (): Promise<void> {
  const inputs: string[][][] = [
    [
      ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
      ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      ['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ],
    [
      ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
      ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      ['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]
  ]

  for (const board of inputs) {
    const result = isValidSudoku(board)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
