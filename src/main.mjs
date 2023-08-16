/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number[]} nums
  * @param {number} k
  * @returns {number[]}
  */
function maxSlidingWindow (nums, k) {
  /** @type {number[]} */
  const queue = []
  /** @type {number[]} */
  const result = []

  for (let i = 0; i < k; i += 1) {
    while (queue.length > 0) {
      const taili = queue.at(-1)
      const tail = nums[taili]
      const num = nums[i]
      if (num >= tail) {
        queue.pop()
      } else {
        break
      }
    }

    queue.push(i)
  }

  result.push(nums[queue[0]])

  for (let i = k; i < nums.length; i += 1) {
    const head = queue[0]
    if (head === (i - k)) {
      queue.shift()
    }

    while (queue.length > 0) {
      const taili = queue.at(-1)
      const tail = nums[taili]
      const num = nums[i]
      if (num >= tail) {
        queue.pop()
      } else {
        break
      }
    }

    queue.push(i)
    result.push(nums[queue[0]])
  }

  return result
}

function main () {
  const inputs = [
    { nums: [1, 3, -1, -3, 5, 3, 6, 7], k: 3 },
    { nums: [1], k: 1 }
  ]

  for (const { nums, k } of inputs) {
    const result = maxSlidingWindow(nums, k)
    console.log(result)
  }
}
main()
