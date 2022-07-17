export function kInversePairs (n, k) {
  let dp = new Array(k + 1).fill(0)
  const MODULO = 1000000007

  for (let i = 1; i <= n; i += 1) {
    const temp = new Array(k + 1).fill(0)
    temp[0] = 1
    for (let j = 1; j <= k; j += 1) {
      let v = dp[j] + MODULO
      if (j >= i) {
        v -= dp[j - i]
      }
      v %= MODULO
      temp[j] = (temp[j - 1] + v) % MODULO
    }
    dp = temp
  }

  let result = dp[k] + MODULO
  if (k > 0) {
    result -= dp[k - 1]
  }

  return result % MODULO
}
