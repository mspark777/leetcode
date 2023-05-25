/**
  * @param {number} n
  * @param {number} k
  * @param {number} maxPts
  * @returns {number}
  */
function new21Game (n, k, maxPts) {
  if (k === 0) {
    return 1
  } else if (n >= (k + maxPts)) {
    return 1
  }

  /** @type {number[]} */
  const dp = new Array(n + 1).fill(0)
  dp[0] = 1
  let sum = 1
  let result = 0
  for (let i = 1; i <= n; i += 1) {
    dp[i] = sum / maxPts
    if (i < k) {
      sum += dp[i]
    } else {
      result += dp[i]
    }

    if ((i - maxPts) >= 0) {
      sum -= dp[i - maxPts]
    }
  }
  return result
}

async function main () {
  const inputs = [
    [10, 1, 10],
    [6, 1, 10],
    [21, 17, 10]
  ]

  for (const [n, k, maxPts] of inputs) {
    const result = new21Game(n, k, maxPts)
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
