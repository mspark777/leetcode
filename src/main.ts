import '@total-typescript/ts-reset'

function backtrack (permutations: Set<number>, nums: number[], results: number[][]): void {
  if (permutations.size === nums.length) {
    results.push([...permutations])
    return
  }

  for (const num of nums) {
    if (permutations.has(num)) {
      continue
    }

    permutations.add(num)
    backtrack(permutations, nums, results)
    permutations.delete(num)
  }
}

function permute (nums: number[]): number[][] {
  const results: number[][] = []
  const permutations = new Set<number>()
  backtrack(permutations, nums, results)

  return results
}

function main (): void {
  const inputs: number[][] = [
    [1, 2, 3],
    [0, 1],
    [1]
  ]

  for (const nums of inputs) {
    const result = permute(nums)
    console.log(result)
  }
}
main()
