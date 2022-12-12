/**
 * @param {number} n
 * @returns {number}
*/
function climbStairs (n) {
  let n0 = 1
  let n1 = 1

  for (let i = 1; i < n; i += 1) {
    const sum = n0 + n1
    n0 = n1
    n1 = sum
  }

  return n1
}

async function main () {
  const inputs = [
    2, 3
  ]

  for (const n of inputs) {
    const result = climbStairs(n)
    console.log(result)
  }
}
main().catch(e => {
  console.error(e)
  process.exit(1)
})
