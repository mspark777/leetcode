function swap (nums: number[], i: number, j: number): void {
  const temp = nums[i]
  nums[i] = nums[j]
  nums[j] = temp
}

function reverse (nums: number[], start: number): void {
  let i = start
  let j = nums.length - 1
  while (i < j) {
    swap(nums, i, j)
    i += 1
    j -= 1
  }
}

function nextPermutation (nums: number[]): void {
  let i = nums.length - 2
  while ((i >= 0) && nums[i] >= nums[i + 1]) {
    i -= 1
  }

  if (i >= 0) {
    let j = nums.length - 1
    while (nums[i] >= nums[j]) {
      j -= 1
    }

    swap(nums, i, j)
  }

  reverse(nums, i + 1)
}

interface Input {
  readonly nums: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      nums: [1, 2, 3]
    },
    {
      nums: [3, 2, 1]
    },
    {
      nums: [1, 1, 5]
    }
  ]

  for (const { nums } of inputs) {
    nextPermutation(nums)
    console.log(nums)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
