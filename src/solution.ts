export function isInterleave (s1: string, s2: string, s3: string): boolean {
  const s1Len = s1.length
  const s2Len = s2.length
  const s3Len = s3.length
  if ((s1Len + s2Len) !== s3Len) {
    return false
  }

  const dp = new Array<boolean>(s2Len + 1)

  for (let i = 0; i <= s1Len; i += 1) {
    for (let j = 0; j <= s2Len; j += 1) {
      if ((i === 0) && (j === 0)) {
        dp[j] = true
      } else if (i === 0) {
        dp[j] = dp[j - 1] && s2.charCodeAt(j - 1) === s3.charCodeAt(i + j - 1)
      } else if (j === 0) {
        dp[j] = dp[j] && s1.charCodeAt(i - 1) === s3.charCodeAt(i + j - 1)
      } else {
        dp[j] = (dp[j] && s1.charCodeAt(i - 1) === s3.charCodeAt(i + j - 1)) ||
          (dp[j - 1] && s2.charCodeAt(j - 1) === s3.charCodeAt(i + j - 1))
      }
    }
  }

  return dp[s2Len]
}
