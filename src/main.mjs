/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
function uniquePaths (m, n) {
  let total = m + n - 2
  const r = Math.min(m, n) - 1

  let steps = 1

  for (let i = 1; i <= r; i += 1, total -= 1) {
    steps = Math.trunc(steps * total / i)
  }

  return steps
}

async function main () {
  const inputs = [
    {
      m: 3,
      n: 7
    },
    {
      m: 3,
      n: 2
    }
  ]

  for (const input of inputs) {
    const { m, n } = input
    const result = uniquePaths(m, n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
