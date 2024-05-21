/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number} i
 * @param {number[]} nums
 * @param {number[]} temp
 * @param {number[][]} result
 * @returns {void}
 */
function solve(i, nums, temp, result) {
  result.push([...temp]);

  for (let j = i; j < nums.length; j += 1) {
    temp.push(nums[j]);
    solve(j + 1, nums, temp, result);
    temp.pop();
  }
}

/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function subsets(nums) {
  const result = [];
  solve(0, nums, [], result);

  return result;
}

function main() {
  const inputs = [[1, 2, 3], [0]];

  for (const nums of inputs) {
    const result = subsets(nums);
    console.log(result);
  }
}
main();
