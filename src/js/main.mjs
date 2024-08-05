/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string[]} arr
 * @param {number} k
 * @return {string}
 */
function kthDistinct(arr, k) {
  const frequency_map = new Map();
  for (const s of arr) {
    const count = frequency_map.get(s) ?? 0;
    frequency_map.set(s, count + 1);
  }

  for (const s of arr) {
    if (frequency_map.get(s) === 1) {
      k -= 1;
    }

    if (k === 0) {
      return s;
    }
  }

  return "";
}

function main() {
  const inputs = [
    [["d", "b", "c", "b", "c", "a"], 2],
    [["aaa", "aa", "a"], 1],
    [["a", "b", "a"], 3],
  ];

  for (const [arr, k] of inputs) {
    const result = kthDistinct(arr, k);
    console.log(result);
  }
}
main();
