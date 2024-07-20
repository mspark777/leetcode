/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} rowSum
 * @param {number[]} colSum
 * @return {number[][]}
 */
function restoreMatrix(rowSum, colSum) {
  const N = rowSum.length;
  const M = colSum.length;

  const result = [];
  for (let i = 0; i < N; i += 1) {
    const row = [];
    for (let j = 0; j < M; j += 1) {
      const rsum = rowSum[i];
      const csum = colSum[j];

      const cell = Math.min(rsum, csum);
      row.push(cell);
      rowSum[i] -= cell;
      colSum[j] -= cell;
    }
    result.push(row);
  }

  return result;
}

function main() {
  const inputs = [
    [
      [3, 8],
      [4, 7],
    ],
    [
      [5, 7, 10],
      [8, 6, 8],
    ],
  ];

  for (const [rowSum, colSum] of inputs) {
    const result = restoreMatrix(rowSum, colSum);
    console.log(result);
  }
}
main();
