import '@total-typescript/ts-reset'

function idx (n: number, r: number, c: number): number {
  return (r * n) + c
}

function strangePrinter (s: string): number {
  const n = s.length
  const dp = new Array<number>(n * n).fill(0)
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
          const lmin = dp[idx(n, left, right)] as number
          const rmin = 1 + (dp[idx(n, j, i)] as number) + (dp[idx(n, i + 1, right)] as number)
          dp[idx(n, left, right)] = Math.min(lmin, rmin)
        }
      }

      if (j === -1) {
        dp[idx(n, left, right)] = 0
      }
    }
  }

  return (dp[idx(n, 0, n - 1)] as number) + 1
}

function main (): void {
  const inputs: string[] = [
    'aaabbb', 'aba'
  ]

  for (const s of inputs) {
    const result = strangePrinter(s)
    console.log(result)
  }
}
main()
