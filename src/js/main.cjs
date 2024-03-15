/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} nums
 * @return {number[]}
 */
function productExceptSelf(nums) {
  const result = new Array(nums.length).fill(1);
  let left = 1;
  let right = 1;
  for (let i = 0; i < nums.length; i++) {
    r = nums.length - 1;
    result[i] *= left;
    result[r - i] *= right;
    left *= nums[i];
    right *= nums[r - i];
  }

  return result;
}

function main() {
  const inputs = [
    [1, 2, 3, 4],
    [-1, 1, 0, -3, 3],
  ];

  for (const nums of inputs) {
    const result = productExceptSelf(nums);
    console.log(result);
  }
}
main();
