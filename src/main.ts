import '@total-typescript/ts-reset'

function findKthLargest (nums: number[], k: number): number {
  nums.sort((a, b) => b - a)
  return nums[k - 1] as number
}

interface Input {
  readonly nums: number[]
  readonly k: number
}

function main (): void {
  const inputs: Input[] = [
    { nums: [3, 2, 1, 5, 6, 4], k: 2 },
    { nums: [3, 2, 3, 1, 2, 4, 5, 5, 6], k: 4 }
  ]

  for (const { nums, k } of inputs) {
    const result = findKthLargest(nums, k)
    console.log(result)
  }
}
main()
