import '@total-typescript/ts-reset'

function backtrack (candidates: number[], combinations: number[], remain: number, cur: number, results: number[][]): void {
  if (remain === 0) {
    results.push(combinations.slice())
    return
  }

  for (let next = cur; next < candidates.length; next += 1) {
    if ((next > cur) && (candidates[next] === candidates[next - 1])) {
      continue
    }

    const pick = candidates[next] as number
    const nextRemain = remain - pick
    if (nextRemain < 0) {
      break
    }

    combinations.push(pick)
    backtrack(candidates, combinations, nextRemain, next + 1, results)
    combinations.pop()
  }
}

function combinationSum2 (candidates: number[], target: number): number[][] {
  const results: number[][] = []
  const combinations: number[] = []

  candidates.sort((a, b) => a - b)
  backtrack(candidates, combinations, target, 0, results)
  return results
}

interface Input {
  readonly candidates: number[]
  readonly target: number
}

function main (): void {
  const inputs: Input[] = [
    { candidates: [10, 1, 2, 7, 6, 1, 5], target: 8 },
    { candidates: [2, 5, 2, 1, 2], target: 5 }
  ]

  for (const { candidates, target } of inputs) {
    const result = combinationSum2(candidates, target)
    console.log(result)
  }
}
main()
