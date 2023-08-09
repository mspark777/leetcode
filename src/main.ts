import '@total-typescript/ts-reset'

function countValidPairs (nums: number[], threshold: number): number {
  let index = 0
  let count = 0
  const last = nums.length - 1
  while (index < last) {
    const n0 = nums[index] as number
    const n1 = nums[index + 1] as number
    const diff = n1 - n0
    if (diff <= threshold) {
      count += 1
      index += 1
    }

    index += 1
  }

  return count
}

function minimizeMax (nums: number[], p: number): number {
  nums.sort((a, b) => a - b)

  let left = 0
  let right = (nums.at(-1) as number) - (nums.at(0) as number)
  while (left < right) {
    const mid = Math.trunc((left + right) / 2)

    if (countValidPairs(nums, mid) >= p) {
      right = mid
    } else {
      left = mid + 1
    }
  }

  return left
}

interface Input {
  readonly nums: number[]
  readonly p: number
}

function main (): void {
  const inputs: Input[] = [
    { nums: [10, 1, 2, 7, 1, 3], p: 2 },
    { nums: [4, 2, 1, 2], p: 1 }
  ]

  for (const { nums, p } of inputs) {
    const result = minimizeMax(nums, p)
    console.log(result)
  }
}
main()
