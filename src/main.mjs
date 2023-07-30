/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number} n
  * @param {number} r
  * @param {number} c
  * @returns {number}
  */
function idx (n, r, c) {
  return (r * n) + c
}

/**
  * @param {string} s
  * @returns {number}
  */
function strangePrinter (s) {
  const n = s.length
  /** @type {number[]} */
  const dp = new Array(n * n).fill(0)
  for (let l = 1; l <= n; l += 1) {
    for (let left = 0; left <= n - l; left += 1) {
      const right = left + l - 1
      let j = -1
      dp[idx(n, left, right)] = n
      for (let i = left; i < right; i += 1) {
        if ((s.charAt(i) !== s.charAt(right)) && (j === -1)) {
          j = i
        }

        if (j !== -1) {
          const lmin = dp[idx(n, left, right)]
          /** @type {number} */
          const rmin0 = dp[idx(n, j, i)]
          /** @type {number} */
          const rmin1 = dp[idx(n, i + 1, right)]
          const rmin = 1 + rmin0 + rmin1
          dp[idx(n, left, right)] = Math.min(lmin, rmin)
        }
      }

      if (j === -1) {
        dp[idx(n, left, right)] = 0
      }
    }
  }

  /** @type {number} */
  const result = dp[idx(n, 0, n - 1)]
  return result + 1
}

function main () {
  const inputs = [
    'aaabbb',
    'aba'
  ]

  for (const s of inputs) {
    const result = strangePrinter(s)
    console.log(result)
  }
}
main()
