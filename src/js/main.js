/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unused-vars */

/**
 * @param {Array} arr
 * @param {number} size
 * @return {Array}
 */
var chunk = function (arr, size) {
  const result = [];
  for (let i = 0; i < arr.length; i += size) {
    const chunk = [];
    for (let j = 0; j < size; j += 1) {
      const k = i + j;
      if (k < arr.length) {
        chunk.push(arr[k]);
      } else {
        break;
      }
    }

    result.push(chunk);
  }

  return result;
};

/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var minimumDifference = function (nums, k) {
  if (nums.length < k) {
    return 0;
  }

  nums.sort((a, b) => a - b);
  let result = Number.MAX_SAFE_INTEGER;

  for (let i = 0; i < nums.length - k + 1; i++) {
    result = Math.min(result, nums[i + k - 1] - nums[i]);
  }

  return result;
};

/**
 * @typedef Input
 * @property {number[]} nums
 * @property {number} k
 */

/**
 * @return {undefined}
 */
function main() {
  /** @type Input[] */
  const inputs = [
    {
      nums: [90],
      k: 1,
    },
    {
      nums: [9, 4, 1, 7],
      k: 2,
    },
  ];

  for (const input of inputs) {
    const result = minimumDifference(input.nums, input.k);
    console.log(result);
  }
}
main();
