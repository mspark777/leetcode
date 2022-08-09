/**
 * @param {number[]} arr
 * @return {number}
 */
function numFactoredBinaryTrees (arr) {
  const MOD = 1000000007n
  const LEN = arr.length

  arr.sort((a, b) => a - b)

  const dp = new Array(LEN).fill(1n)
  const index = new Map()

  for (let i = 0; i < LEN; i += 1) {
    const key = BigInt(arr[i])
    index.set(key, BigInt(i))
  }

  for (let i = 0; i < LEN; i += 1) {
    const parent = BigInt(arr[i])
    for (let j = 0; j < i; j += 1) {
      const left = BigInt(arr[j])
      if ((parent % left) === 0n) {
        const right = parent / left
        if (index.has(right)) {
          const r = Number(index.get(right))
          dp[i] = (dp[i] + dp[j] * dp[r]) % MOD
        }
      }
    }
  }

  const result = dp.reduce((acc, cur) => acc + cur, 0n)
  return Number(result % MOD)
}

async function main () {
  const inputs = [
    {
      arr: [2, 4]
    },
    {
      arr: [2, 4, 5, 10]
    }
  ]

  for (const { arr } of inputs) {
    const result = numFactoredBinaryTrees(arr)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
