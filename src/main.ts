function convert (s: string, numRows: number): string {
  if (numRows <= 1) {
    return s
  }

  const lastRow = numRows - 1
  let row = 0
  let down = true

  const result = Array.from(new Array(numRows), () => new Array<string>())
  for (const ch of s) {
    result[row].push(ch)
    if (row === lastRow) {
      down = false
    } else if (row === 0) {
      down = true
    }

    if (down) {
      row += 1
    } else {
      row -= 1
    }
  }

  return result.map(r => r.join('')).join('')
}

interface Input {
  readonly s: string
  readonly numRows: number
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    { s: 'PAYPALISHIRING', numRows: 3 },
    { s: 'PAYPALISHIRING', numRows: 4 },
    { s: 'A', numRows: 1 },
    { s: 'AB', numRows: 1 }
  ]

  for (const { s, numRows } of inputs) {
    const result = convert(s, numRows)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
