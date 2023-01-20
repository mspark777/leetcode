function backtrack (nums: number[], index: number, sequence: number[], result: Map<string, number[]>): void {
  if (index === nums.length) {
    if (sequence.length >= 2) {
      const key = sequence.join()
      result.set(key, sequence.slice())
    }
  } else {
    const num = nums[index]
    const lastseq = sequence.at(-1) ?? num

    if (lastseq <= num) {
      sequence.push(num)
      backtrack(nums, index + 1, sequence, result)
      sequence.pop()
    }
    backtrack(nums, index + 1, sequence, result)
  }
}

function findSubsequences (nums: number[]): number[][] {
  const result = new Map<string, number[]>()
  const sequence = new Array<number>()
  backtrack(nums, 0, sequence, result)
  return [...result.values()]
}

async function main (): Promise<void> {
  const inputs: number[][] = [
    [4, 6, 7, 7],
    [4, 4, 3, 2, 1]
  ]

  for (const nums of inputs) {
    const result = findSubsequences(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
