function longestPalindromeSubseq (s: string): number {
  const n = s.length
  const dp = new Array<number>(n).fill(0)
  let dpPrev = new Array<number>(n).fill(0)

  for (let i = n - 1; i >= 0; i -= 1) {
    dp[i] = 1
    for (let j = i + 1; j < n; j += 1) {
      if (s.charAt(i) === s.charAt(j)) {
        dp[j] = dpPrev[j - 1] + 2
      } else {
        dp[j] = Math.max(dpPrev[j], dp[j - 1])
      }
    }
    dpPrev = dp.slice()
  }

  return dp[n - 1]
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'bbbab',
    'cbbd'
  ]

  for (const s of inputs) {
    const result = longestPalindromeSubseq(s)
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
