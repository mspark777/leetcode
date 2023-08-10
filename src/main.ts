import '@total-typescript/ts-reset'

function search (nums: number[], target: number): boolean {
  if (nums.length < 1) {
    return false
  }

  let left = 0
  let right = nums.length
  while (left < right) {
    const mid = Math.trunc((left + right) / 2)
    const mnum = nums[mid] as number
    if (mnum === target) {
      return true
    }

    const lnum = nums[left] as number
    if (lnum === mnum) {
      left += 1
      continue
    }

    const pivotArray = lnum <= mnum
    const targetArray = lnum <= target
    if (pivotArray !== targetArray) {
      if (pivotArray) {
        left = mid + 1
      } else {
        right = mid
      }
    } else {
      if (mnum < target) {
        left = mid + 1
      } else {
        right = mid
      }
    }
  }

  return false
}

interface Input {
  readonly nums: number[]
  readonly target: number
}

function main (): void {
  const inputs: Input[] = [
    { nums: [2, 5, 6, 0, 0, 1, 2], target: 0 },
    { nums: [2, 5, 6, 0, 0, 1, 2], target: 3 },
    { nums: [1, 0, 1, 1, 1], target: 0 }
  ]

  for (const { nums, target } of inputs) {
    const result = search(nums, target)
    console.log(result)
  }
}
main()
