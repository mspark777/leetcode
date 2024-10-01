/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} arr
 * @param {number} k
 * @return {boolean}
 */
function canArrange(arr, k) {
  /** @type {Map<number, number>} */
  const remainderCounts = new Map();

  for (const i of arr) {
    const idx = ((i % k) + k) % k;
    const rem = remainderCounts.get(idx) ?? 0;
    remainderCounts.set(idx, rem + 1);
  }

  for (const i of arr) {
    const rem = ((i % k) + k) % k;

    if (rem == 0) {
      const cnt = remainderCounts.get(rem);
      if (cnt == null) {
        continue;
      } else if (cnt % 2 === 1) {
        return false;
      }
    } else if (remainderCounts.get(rem) !== remainderCounts.get(k - rem)) {
      return false;
    }
  }

  return true;
}

function main() {
  const inputs = [
    [[1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5],
    [[1, 2, 3, 4, 5, 6], 7],
    [[1, 2, 3, 4, 5, 6], 10],
  ];

  for (const [arr, k] of inputs) {
    const result = canArrange(arr, k);
    console.log(result);
  }
}
main();
