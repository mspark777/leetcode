/**
 * @param {number} n
 * @param {number} t0
 * @param {number} t1
 * @param {number} t2
 * @returns {number}
 */
function recursive (n, t0, t1, t2) {
  return n > 2 ? recursive(n - 1, t1, t2, t0 + t1 + t2) : t2
}

/**
 * @param {number} n
 * @returns {number}
 */
function tribonacci (n) {
  if (n === 0) {
    return 0
  } else if (n < 3) {
    return 1
  }

  return recursive(n, 0, 1, 1)
}

async function main () {
  const inputs = [
    4, 25
  ]

  for (const n of inputs) {
    const result = tribonacci(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
