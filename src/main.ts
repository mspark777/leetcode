function increasingTriplet (nums: number[]): boolean {
  if (nums.length < 3) {
    return false
  }

  let min = Number.MAX_SAFE_INTEGER
  let middle = Number.MAX_SAFE_INTEGER
  for (const n of nums) {
    if (n <= min) {
      min = n
    } else if (n <= middle) {
      middle = n
    } else {
      return true
    }
  }

  return false
}

async function main (): Promise<void> {
  const inputs: number[][] = [
    [1, 2, 3, 4, 5],
    [5, 4, 3, 2, 1],
    [2, 1, 5, 0, 4, 6],
    [2, 6, 1, 8]
  ]

  for (const nums of inputs) {
    const result = increasingTriplet(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
