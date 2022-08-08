function binarySearch (nums: number[], num: number): number {
  let left = 0
  let right = nums.length - 1
  while (left <= right) {
    const mid = Math.trunc((right + left) / 2)
    if (nums[mid] < num) {
      left = mid + 1
    } else {
      right = mid - 1
    }
  }

  return left
}

function lengthOfLIS (nums: number[]): number {
  const result: number[] = [nums[0]]

  for (let i = 1; i < nums.length; i += 1) {
    const num = nums[i]
    if (num > result.at(-1)!) {
      result.push(num)
    } else {
      const index = binarySearch(result, num)
      result[index] = num
    }
  }

  return result.length
}

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [10, 9, 2, 5, 3, 7, 101, 18]
    },
    {
      nums: [0, 1, 0, 3, 2, 3]
    },
    {
      nums: [7, 7, 7, 7, 7, 7, 7]
    }
  ]

  for (const { nums } of inputs) {
    const result = lengthOfLIS(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
