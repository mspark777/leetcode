/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number} i
  * @param {number} j
  * @param {Map<number, Map<number, number>>} dp
  * @returns {number}
  */
function calculateDP (i, j, dp) {
  /** @type {number} */
  const dp0 = dp.get(Math.max(0, i - 4))?.get(j)
  /** @type {number} */
  const dp1 = dp.get(Math.max(0, i - 3))?.get(j - 1)
  /** @type {number} */
  const dp2 = dp.get(Math.max(0, i - 2))?.get(Math.max(0, j - 2))
  /** @type {number} */
  const dp3 = dp.get(i - 1)?.get(Math.max(0, j - 3))
  /** @type {number} */
  const sum = dp0 + dp1 + dp2 + dp3
  return sum / 4
}

/**
  * @param {number} n
  * @returns {number}
  */
function soupServings (n) {
  /** @type {Map<number, Map<number, number>>} */
  const dp = new Map()
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

    const dpk = dp.get(k)?.get(k)
    if (dpk > (1 - 1e-5)) {
      return 1
    }
  }

  return dp.get(m)?.get(m)
}

function main () {
  const inputs = [
    50, 100
  ]

  for (const n of inputs) {
    const result = soupServings(n)
    console.log(result)
  }
}
main()
