/**
  * @param {number} x
  * @param {number} y
  * @returns {number}
  */
function floowMod (x, y) {
  return ((x % y) + y) % y
}

function generateMatrix (n) {
  const result = Array.from(new Array(n), () => new Array(n).fill(0))
  let cnt = 1
  const dir = [[0, 1], [1, 0], [0, -1], [-1, 0]]
  let d = 0
  let row = 0
  let col = 0

  while (cnt <= (n * n)) {
    result[row][col] = cnt
    cnt += 1
    const r = floowMod(row + dir[d][0], n)
    const c = floowMod(col + dir[d][1], n)

    if (result[r][c] !== 0) {
      d = (d + 1) % 4
    }

    row += dir[d][0]
    col += dir[d][1]
  }

  return result
}

async function main () {
  const inputs = [
    3, 1
  ]

  for (const n of inputs) {
    const result = generateMatrix(n)
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
