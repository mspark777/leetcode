/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} nums
 * @return {number}
 */
function maxWidthRamp(nums) {
  const n = nums.length;

  /** @type {number[]} */
  const indices = [];

  for (const [i, n] of nums.entries()) {
    const top = indices.at(-1);
    if (top == null) {
      indices.push(i);
    } else if (nums[top] > n) {
      indices.push(i);
    }
  }

  let result = 0;
  for (let i = n - 1; i >= 0; i -= 1) {
    while (indices.length > 0 && nums.at(indices.at(-1)) <= nums.at(i)) {
      result = Math.max(result, i - indices.at(-1));
      indices.pop();
    }
  }

  return result;
}

function main() {
  const inputs = [
    [6, 0, 8, 2, 1, 5],
    [9, 8, 1, 0, 1, 9, 4, 0, 4, 1],
  ];

  for (const nums of inputs) {
    const result = maxWidthRamp(nums);
    console.log(result);
  }
}
main();
