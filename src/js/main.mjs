/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} arr
 * @return {number[]}
 */
function arrayRankTransform(arr) {
  const unique = [...new Set(arr)].sort((l, r) => l - r);

  /** @type {Map<number, number>} */
  const ranks = new Map();
  for (const [i, n] of unique.entries()) {
    ranks.set(n, i + 1);
  }

  return arr.map((n) => ranks.get(n));
}

function main() {
  const inputs = [
    [40, 10, 20, 30],
    [100, 100, 100],
    [37, 12, 28, 9, 100, 56, 80, 5, 12],
  ];

  for (const arr of inputs) {
    const result = arrayRankTransform(arr);
    console.log(result);
  }
}
main();
