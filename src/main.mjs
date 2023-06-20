/**
  * @param {number[]} nums
  * @param {number} k
  * @returns {number[]}
  */
function getAverages (nums, k) {
  if (k < 1) {
    return nums
  }

  const result = nums.map(() => -1)
  const windowLen = (k * 2) + 1
  if (windowLen > nums.length) {
    return result
  }

  let windowSum = nums.slice(0, windowLen).reduce((acc, cur) => acc + cur)
  result[k] = Math.trunc(windowSum / windowLen)

  for (let i = windowLen; i < nums.length; i += 1) {
    windowSum = windowSum - nums[i - windowLen] + nums[i]
    result[i - k] = Math.trunc(windowSum / windowLen)
  }

  return result
}

function main () {
  const inputs = [
    { nums: [7, 4, 3, 9, 1, 8, 5, 2, 6], k: 3 },
    { nums: [100000], k: 0 },
    { nums: [8], k: 100000 }
  ]

  for (const { nums, k } of inputs) {
    const result = getAverages(nums, k)
    console.log(result)
  }
}
main()
