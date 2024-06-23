/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} nums
 * @param {number} limit
 * @return {number}
 */
function longestSubarray(nums, limit) {
  const maxDeque = [];
  const minDeque = [];
  let left = 0;
  let result = 0;

  for (const [right, rnum] of nums.entries()) {
    while (maxDeque.length > 0 && maxDeque.at(-1) < rnum) {
      maxDeque.pop();
    }
    maxDeque.push(rnum);

    while (minDeque.length > 0 && minDeque.at(-1) > rnum) {
      minDeque.pop();
    }
    minDeque.push(rnum);

    while (maxDeque[0] - minDeque[0] > limit) {
      const lnum = nums[left];
      if (maxDeque[0] === lnum) {
        maxDeque.shift();
      }

      if (minDeque[0] === lnum) {
        minDeque.shift();
      }
      left += 1;
    }

    result = Math.max(result, right - left + 1);
  }

  return result;
}

function main() {
  const inputs = [
    [[8, 2, 4, 7], 4],
    [[10, 1, 2, 4, 7, 2], 5],
    [[4, 2, 2, 2, 4, 4, 2, 2], 0],
  ];

  for (const [nums, limit] of inputs) {
    const result = longestSubarray(nums, limit);
    console.log(result);
  }
}
main();
