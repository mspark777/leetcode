import '@total-typescript/ts-reset'

function f (piles: number[], dp: number[][][], p: number, i: number, m: number): number {
  if (i === piles.length) {
    return 0
  } else if (dp[p][i][m] !== -1) {
    return dp[p][i][m]
  }

  let result = p === 1 ? 1000000 : -1
  let s = 0
  for (let x = 1; x <= Math.min(2 * m, piles.length - i); x += 1) {
    s += piles[i + x - 1]
    if (p === 0) {
      const r = f(piles, dp, 1, i + x, Math.max(m, x))
      result = Math.max(result, s + r)
    } else {
      const r = f(piles, dp, 0, i + x, Math.max(m, x))
      result = Math.min(result, r)
    }
  }

  dp[p][i][m] = result
  return result
}

function stoneGameII (piles: number[]): number {
  const dp = new Array<number[][]>(2)
  for (let p = 0; p < 2; p += 1) {
    dp[p] = new Array(piles.length + 1)
    for (let i = 0; i <= piles.length; i += 1) {
      dp[p][i] = new Array(piles.length + 1).fill(-1)
    }
  }
  return f(piles, dp, 0, 0, 1)
}

async function main (): Promise<void> {
  const inputs = [
    [2, 7, 9, 4, 4],
    [1, 2, 3, 4, 5, 100]
  ]

  for (const piles of inputs) {
    const result = stoneGameII(piles)
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
