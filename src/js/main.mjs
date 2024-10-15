/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} nums
 * @return {number}
 */
function findShortestSubArray(nums) {
  /** @type {Map<number, number>} */
  const lefts = new Map();
  /** @type {Map<number, number>} */
  const rights = new Map();
  /** @type {Map<number, number>} */
  const counts = new Map();

  for (const [i, num] of nums.entries()) {
    if (!lefts.has(num)) {
      lefts.set(num, i);
    }

    rights.set(num, i);

    const count = counts.get(num) ?? 0;
    counts.set(num, count + 1);
  }

  let result = nums.length;
  const degree = Math.max(...counts.values());
  for (const [num, count] of counts) {
    if (count === degree) {
      const left = lefts.get(num);
      const right = rights.get(num);
      result = Math.min(result, right - left + 1);
    }
  }

  return result;
}

function main() {
  const inputs = [
    [1, 2, 2, 3, 1],
    [1, 2, 2, 3, 1, 4, 2],
  ];

  for (const nums of inputs) {
    const result = findShortestSubArray(nums);
    console.log(result);
  }
}
main();
