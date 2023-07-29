import '@total-typescript/ts-reset'

function calculateDP (i: number, j: number, dp: Map<number, Map<number, number>>): number {
  const dp0 = dp.get(Math.max(0, i - 4))?.get(j) as number
  const dp1 = dp.get(Math.max(0, i - 3))?.get(j - 1) as number
  const dp2 = dp.get(Math.max(0, i - 2))?.get(Math.max(0, j - 2)) as number
  const dp3 = dp.get(i - 1)?.get(Math.max(0, j - 3)) as number
  const sum = dp0 + dp1 + dp2 + dp3
  return sum / 4
}

function soupServings (n: number): number {
  const dp = new Map<number, Map<number, number>>()
  dp.set(0, new Map([[0, 0.5]]))

  const m = Math.ceil(n / 25)
  for (let k = 1; k <= m; k += 1) {
    dp.set(k, new Map())
    dp.get(0)?.set(k, 1)
    dp.get(k)?.set(0, 0)

    for (let j = 1; j <= k; j += 1) {
      dp.get(j)?.set(k, calculateDP(j, k, dp))
      dp.get(k)?.set(j, calculateDP(k, j, dp))
    }

    const dpk = dp.get(k)?.get(k) as number
    if (dpk > (1 - 1e-5)) {
      return 1
    }
  }

  return dp.get(m)?.get(m) as number
}

function main (): void {
  const inputs: number[] = [
    50, 100
  ]

  for (const n of inputs) {
    const result = soupServings(n)
    console.log(result)
  }
}
main()
