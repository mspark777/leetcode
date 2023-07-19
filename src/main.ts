import '@total-typescript/ts-reset'

function solve (i: number, candidates: number[], temp: number[], target: number, result: number[][]): void {
  if (target === 0) {
    result.push(temp.slice())
    return
  }

  if (target < 0) {
    return
  }

  if (i >= candidates.length) {
    return
  }

  solve(i + 1, candidates, temp, target, result)

  const candidate = candidates[i] as number
  temp.push(candidate)
  solve(i, candidates, temp, target - candidate, result)
  temp.pop()
}

function combinationSum (candidates: number[], target: number): number[][] {
  const result: number[][] = []

  solve(0, candidates, [], target, result)
  return result
}

interface Input {
  readonly candidates: number[]
  readonly target: number
}

function main (): void {
  const inputs: Input[] = [
    { candidates: [2, 3, 6, 7], target: 7 },
    { candidates: [2, 3, 5], target: 8 },
    { candidates: [2], target: 1 }
  ]

  for (const { candidates, target } of inputs) {
    const result = combinationSum(candidates, target)
    console.log(result)
  }
}
main()
