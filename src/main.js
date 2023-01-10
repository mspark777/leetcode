/**
 * @param {number} n
 * @param {number[]} memo
 * @returns {number}
 */
function solve (n, memo) {
  if (n === 0) {
    return 0
  } else if (n === 1) {
    return 1
  }

  if (memo[n] !== 0) {
    return memo[n]
  }

  if ((n % 2) === 0) {
    memo[n] = solve(Math.trunc(n / 2), memo)
  } else {
    memo[n] = solve(Math.trunc(n / 2), memo) + 1
  }

  return memo[n]
}

/**
 * @param {number} n
 * @returns {number[]}
 */
function countBits (n) {
  const result = new Array(n + 1).fill(0)

  for (let i = 1; i <= n; i += 1) {
    result[i] = solve(i, result)
  }

  return result
}

async function main () {
  const inputs = [
    2, 5
  ]

  for (const n of inputs) {
    const result = countBits(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
