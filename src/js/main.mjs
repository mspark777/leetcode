/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} arr1
 * @param {number[]} arr2
 * @return {number[]}
 */
function relativeSortArray(arr1, arr2) {
  const indexMap = new Map();
  for (const [i, n] of arr2.entries()) {
    indexMap.set(n, i);
  }

  return arr1.sort((left, right) => {
    const left2 = indexMap.get(left);
    const right2 = indexMap.get(right);
    if (left2 != null && right2 != null) {
      return left2 - right2;
    } else if (left2 == null && right2 == null) {
      return left - right;
    } else if (left2 != null) {
      return -1;
    } else {
      return 1;
    }
  });
}

function main() {
  const inputs = [
    [
      [2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
      [2, 1, 4, 3, 9, 6],
    ],
    [
      [28, 6, 22, 8, 44, 17],
      [22, 28, 8, 6],
    ],
  ];

  for (const [arr1, arr2] of inputs) {
    const result = relativeSortArray(arr1, arr2);
    console.log(result);
  }
}
main();
