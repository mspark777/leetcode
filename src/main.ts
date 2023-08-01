import '@total-typescript/ts-reset'

function backtrack (start: number, combinations: number[], results: number[][], n: number, k: number): void {
  if (combinations.length === k) {
    results.push([...combinations])
    return
  }

  for (let i = start; i <= n; i += 1) {
    combinations.push(i)
    backtrack(i + 1, combinations, results, n, k)
    combinations.pop()
  }
}

function combine (n: number, k: number): number[][] {
  const results: number[][] = []
  const combinations: number[] = []
  backtrack(1, combinations, results, n, k)

  return results
}

function main (): void {
  const inputs: number[][] = [
    [4, 2],
    [1, 1]
  ]

  for (const [n, k] of inputs) {
    const result = combine(n as number, k as number)
    console.log(result)
  }
}
main()
