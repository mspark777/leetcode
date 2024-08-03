/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} target
 * @param {number[]} arr
 * @return {boolean}
 */
function canBeEqual(target, arr) {
  const frequencies = new Map();
  for (const n of target) {
    const frequency = frequencies.get(n) ?? 0;
    frequencies.set(n, frequency + 1);
  }

  for (const n of arr) {
    const frequency = frequencies.get(n) ?? 0;
    frequencies.set(n, frequency - 1);
  }

  return [...frequencies.values()].every((f) => f === 0);
}

function main() {
  const inputs = [
    [
      [1, 2, 3, 4],
      [2, 4, 1, 3],
    ],
    [[7], [7]],
    [
      [3, 7, 9],
      [3, 7, 11],
    ],
  ];

  for (const [target, arr] of inputs) {
    const result = canBeEqual(target, arr);
    console.log(result);
  }
}
main();
