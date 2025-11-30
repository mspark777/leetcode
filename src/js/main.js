/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unused-vars */

/**
 * @param {number[]} nums
 * @return {void}
 */
var ArrayWrapper = function (nums) {
  this.nums = nums;
};

/**
 * @return {number}
 */
ArrayWrapper.prototype.valueOf = function () {
  return this.nums.reduce((acc, cur) => acc + cur, 0);
};

/**
 * @return {string}
 */
ArrayWrapper.prototype.toString = function () {
  return `[${this.nums.join(",")}]`;
};

/**
 * const obj1 = new ArrayWrapper([1,2]);
 * const obj2 = new ArrayWrapper([3,4]);
 * obj1 + obj2; // 10
 * String(obj1); // "[1,2]"
 * String(obj2); // "[3,4]"
 */

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

  for (let i = 0; i < nums.length - k + 1; i += 1) {
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
