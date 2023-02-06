function shuffle (nums: number[], n: number): number[] {
  const result = new Array<number>(nums.length)

  for (let i = 0; i < n; i += 1) {
    const j = i * 2
    result[j] = nums[i]
    result[j + 1] = nums[n + i]
  }

  return result
}

async function main (): Promise<void> {
  const inputs = [
    { nums: [2, 5, 1, 3, 4, 7], n: 3 },
    { nums: [1, 2, 3, 4, 4, 3, 2, 1], n: 4 }
  ]

  for (const { nums, n } of inputs) {
    const result = shuffle(nums, n)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
