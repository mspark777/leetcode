/**
 * @param {number} n
 * @param {number} k
 * @param {number} target
 * @returns {number}
 */
function numRollsToTarget (n, k, target) {
  const MOD = 1000000007

  const dp = new Array(target + 1).fill(0)
  dp[0] = 1

  for (let i = 1; i <= n; i += 1) {
    for (let j = target; j >= 0; j -= 1) {
      dp[j] = 0

      for (let p = 1; p <= k; p += 1) {
        if (j >= p) {
          dp[j] = (dp[j] + dp[j - p]) % MOD
        } else {
          break
        }
      }
    }
  }

  return dp[target]
}

async function main () {
  const inputs = [
    {
      n: 1,
      k: 6,
      target: 3
    },
    {
      n: 2,
      k: 6,
      target: 7
    },
    {
      n: 30,
      k: 30,
      target: 500
    }
  ]

  for (const { n, k, target } of inputs) {
    const result = numRollsToTarget(n, k, target)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
