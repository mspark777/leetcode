/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} nums
 * @return {number}
 */
function findDuplicate(nums) {
  let slow = nums[0];
  let fast = nums[0];

  do {
    slow = nums[slow];
    fast = nums[nums[fast]];
  } while (slow !== fast);

  slow = nums[0];
  while (slow !== fast) {
    slow = nums[slow];
    fast = nums[fast];
  }

  return slow;
}

function main() {
  const inputs = [
    [1, 3, 4, 2, 2],
    [3, 1, 3, 4, 2],
    [3, 3, 3, 3, 3],
    [2, 5, 9, 6, 9, 3, 8, 9, 7, 1],
  ];

  for (const nums of inputs) {
    const result = findDuplicate(nums);
    console.log(result);
  }
}
main();
