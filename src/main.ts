function minDistance (word1: string, word2: string): number {
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

async function main (): Promise<void> {
  const inputs = [
    { word1: 'sea', word2: 'eat' },
    { word1: 'leetcode', word2: 'etco' },
    { word1: 'ab', word2: 'a' }
  ]

  for (const input of inputs) {
    const result = minDistance(input.word1, input.word2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
