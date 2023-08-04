import '@total-typescript/ts-reset'

function wordBreak (s: string, wordDict: string[]): boolean {
  const words = new Set<string>(wordDict)
  const checks = new Array<boolean>(s.length + 1).fill(false)
  checks[0] = true

  for (let right = 1; right <= s.length; right += 1) {
    for (let left = 0; left < right; left += 1) {
      if (checks[left] !== true) {
        continue
      }

      const sub = s.substring(left, right)
      if (words.has(sub)) {
        checks[right] = true
        break
      }
    }
  }

  return checks[s.length] as boolean
}

interface Input {
  readonly s: string
  readonly wordDict: string[]
}

function main (): void {
  const inputs: Input[] = [
    { s: 'leetcode', wordDict: ['leet', 'code'] },
    { s: 'applepenapple', wordDict: ['apple', 'pen'] },
    { s: 'catsandog', wordDict: ['cats', 'dog', 'sand', 'and', 'cat'] }
  ]

  for (const { s, wordDict } of inputs) {
    const result = wordBreak(s, wordDict)
    console.log(result)
  }
}
main()
