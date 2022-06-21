/**
 * @param {number[]} nums
 * @return {number[]}
 */
export const sortArrayByParity = function (nums) {
  let odd = 0
  let even = 0
  const numsLength = nums.length
  while (true) {
    while (odd < numsLength) {
      if (nums[odd] % 2 === 1) {
        break
      } else {
        odd += 1
      }
    }

    even = odd + 1
    while (even < numsLength) {
      if (nums[even] % 2 === 0) {
        break
      } else {
        even += 1
      }
    }

    if (even < numsLength) {
      const n = nums[odd]
      nums[odd] = nums[even]
      nums[even] = n
    }

    if (even >= numsLength) {
      break
    }
  }

  return nums
}
