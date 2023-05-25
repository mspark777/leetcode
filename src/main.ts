import '@total-typescript/ts-reset'

function new21Game (n: number, k: number, maxPts: number): number {
  if (k === 0) {
    return 1
  } else if (n >= (k + maxPts)) {
    return 1
  }

  const dp = new Array<number>(n + 1).fill(0)
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

async function main (): Promise<void> {
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
