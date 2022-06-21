export function minDistance (word1: string, word2: string): number {
  const dpRowCount = word1.length + 1
  const dpColCount = word2.length + 1
  const dp = new Array(dpRowCount)
  for (let i = 0; i < dpRowCount; i += 1) {
    dp[i] = new Array(dpColCount).fill(0)
  }

  for (let i = 0; i < dpRowCount; i += 1) {
    for (let j = 0; j < dpColCount; j += 1) {
      if ((i === 0) || (j === 0)) {
        dp[i][j] = i + j
      } else if (word1.charCodeAt(i - 1) === word2.charCodeAt(j - 1)) {
        dp[i][j] = dp[i - 1][j - 1]
      } else {
        dp[i][j] = 1 + Math.min(dp[i - 1][j], dp[i][j - 1])
      }
    }
  }

  return dp[word1.length][word2.length]
}

export function minDistance1 (word1: string, word2: string): number {
  const dp = new Array(word2.length + 1).fill(0)

  for (let i = 0; i < word1.length; i += 1) {
    const ch1 = word1.charCodeAt(i)
    let prev = dp[0]
    for (let j = 0; j < word2.length; j += 1) {
      const ch2 = word2.charCodeAt(j)
      const val = Math.max(dp[j + 1], ch1 === ch2 ? prev + 1 : dp[j])
      prev = dp[j + 1]
      dp[j + 1] = val
    }
  }

  return word1.length + word2.length - (2 * dp[word2.length])
}

export function minDistance2 (word1: string, word2: string): number {
  const dp = new Array(word2.length + 1).fill(0)

  for (let i = 0; i < word1.length; i += 1) {
    const ch1 = word1.charCodeAt(i)
    let prev = dp[0]
    for (let j = 0; j < word2.length; j += 1) {
      const ch2 = word2.charCodeAt(j)
      const next = j + 1
      const val = Math.max(dp[next], ch1 === ch2 ? prev + 1 : dp[j])
      prev = dp[next]
      dp[next] = val
    }
  }

  return word1.length + word2.length - (2 * dp[word2.length])
}
