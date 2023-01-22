function dfs (result: string[][], s: string, start: number, current: string[], dp: boolean[][]): void {
  if (start >= s.length) {
    result.push(current.slice())
  }

  for (let end = start; end < s.length; end += 1) {
    const check = (s[start] === s[end]) &&
      (((end - start) <= 2) || dp[start + 1][end - 1])
    if (check) {
      dp[start][end] = true
      current.push(s.substring(start, end + 1))
      dfs(result, s, end + 1, current, dp)
      current.pop()
    }
  }
}

function partition (s: string): string[][] {
  const len = s.length
  const dp = Array.from(new Array<boolean[]>(len), () => new Array<boolean>(len).fill(false))
  const result = new Array<string[]>()
  const current = new Array<string>()
  dfs(result, s, 0, current, dp)
  return result
}

async function main (): Promise<void> {
  const inputs: string[] = [
    'aab',
    'a'
  ]

  for (const s of inputs) {
    const result = partition(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
