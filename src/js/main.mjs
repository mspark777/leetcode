/**
  * @param {boolean} b
  * @returns {number}
  */
function bton (b) {
  return b ? 1 : 0
}

/**
  * @param {string[]} pizza
  * @param {number} k
  * @returns {number}
  */
function ways (pizza, k) {
  const rows = pizza.length
  const cols = pizza[0].length
  const apples = Array.from(new Array(rows + 1), () => new Array(cols + 1).fill(0))
  let f = Array.from(new Array(rows), () => new Array(cols).fill(0))

  for (let row = rows - 1; row >= 0; row -= 1) {
    for (let col = cols - 1; col >= 0; col -= 1) {
      apples[row][col] =
        bton(pizza[row][col] === 'A') +
        apples[row + 1][col] +
        apples[row][col + 1] -
        apples[row + 1][col + 1]
      f[row][col] = bton(apples[row][col] > 0)
    }
  }

  const mod = 1000000007
  for (let remain = 1; remain < k; remain += 1) {
    const g = Array.from(new Array(rows), () => new Array(cols).fill(0))

    for (let row = 0; row < rows; row += 1) {
      for (let col = 0; col < cols; col += 1) {
        for (let nextRow = row + 1; nextRow < rows; nextRow += 1) {
          if (apples[row][col] - apples[nextRow][col] > 0) {
            g[row][col] += f[nextRow][col]
            g[row][col] %= mod
          }
        }

        for (let nextCol = col + 1; nextCol < cols; nextCol += 1) {
          if (apples[row][col] - apples[row][nextCol] > 0) {
            g[row][col] += f[row][nextCol]
            g[row][col] %= mod
          }
        }
      }
    }
    f = g
  }
  return f[0][0]
}

async function main () {
  const inputs = [
    [['A..', 'AAA', '...'], 3],
    [['A..', 'AA.', '...'], 3],
    [['A..', 'A..', '...'], 1]
  ]

  for (const [pizza, k] of inputs) {
    const result = ways(pizza, k)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
