function summaryRanges (nums) {
  const result = []

  for (let i = 0; i < nums.length; i += 1) {
    const head = nums[i]
    while (true) {
      const j = i + 1
      if (j >= nums.length) {
        break
      }

      const cur = nums[i] + 1
      const next = nums[j]
      if (cur !== next) {
        break
      }

      i = j
    }

    const tail = nums[i]
    if (head === tail) {
      result.push(head.toString())
    } else {
      result.push(`${head}->${tail}`)
    }
  }

  return result
}

async function main () {
  const inputs = [
    {
      nums: [0, 1, 2, 4, 5, 7]
    },
    {
      nums: [0, 2, 3, 4, 6, 8, 9]
    }
  ]

  for (const input of inputs) {
    const nums = input.nums
    const result = summaryRanges(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
