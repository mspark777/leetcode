import '@total-typescript/ts-reset'

function get (dp: number[][], r: number, c: number): number {
  const row = dp[r] as number[]
  return row[c] as number
}

function set (dp: number[][], r: number, c: number, v: number): void {
  const row = dp[r] as number[]
  row[c] = v
}

function minDistance (word1: string, word2: string): number {
  const len1 = word1.length
  const len2 = word2.length
  const dp = new Array<number[]>(len1 + 1)
  for (let i = 0; i <= len1; i += 1) {
    dp[i] = new Array(len2 + 1).fill(0)
  }

  for (let i = 0; i <= len1; i += 1) {
    const row = dp[i] as number[]
    row[0] = i
  }

  for (let i = 0; i <= len2; i += 1) {
    const row = dp[0] as number[]
    row[i] = i
  }

  for (let i = 1; i <= len1; i += 1) {
    const ch1 = word1.at(i - 1) as string
    for (let j = 1; j <= len2; j += 1) {
      if (ch1 === word2.at(j - 1)) {
        set(dp, i, j, get(dp, i - 1, j - 1))
      } else {
        const memo = Math.min(
          get(dp, i - 1, j - 1),
          Math.min(
            get(dp, i - 1, j),
            get(dp, i, j - 1)
          )
        )
        set(dp, i, j, memo + 1)
      }
    }
  }

  return get(dp, len1, len2)
}

function main (): void {
  const inputs: Array<[string, string]> = [
    ['horse', 'ros'],
    ['intention', 'execution']
  ]

  for (const [word1, word2] of inputs) {
    const result = minDistance(word1, word2)
    console.log(result)
  }
}
main()
