/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} difficulty
 * @param {number[]} profit
 * @param {number[]} worker
 * @return {number}
 */
function maxProfitAssignment(difficulty, profit, worker) {
  const jobProfile = difficulty
    .map((d, i) => [d, profit[i]])
    .sort((a, b) => {
      const [ad, ap] = a;
      const [bd, bp] = b;
      return ap !== bp ? ad - bd : ap - bp;
    });
  worker.sort((a, b) => a - b);

  let result = 0;
  let maxProfit = 0;
  let index = 0;
  for (const ability of worker) {
    while (index < difficulty.length && ability >= jobProfile[index][0]) {
      maxProfit = Math.max(maxProfit, jobProfile[index][1]);
      index += 1;
    }
    result += maxProfit;
  }

  return result;
}

function main() {
  const inputs = [
    [
      [2, 4, 6, 8, 10],
      [10, 20, 30, 40, 50],
      [4, 5, 6, 7],
    ],
    [
      [85, 47, 57],
      [24, 66, 99],
      [40, 25, 25],
    ],
  ];

  for (const [difficulty, profit, worker] of inputs) {
    const result = maxProfitAssignment(difficulty, profit, worker);
    console.log(result);
  }
}
main();
