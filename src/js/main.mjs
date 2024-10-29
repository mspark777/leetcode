/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[][]} grid
 * @return {number}
 */
function maxMoves(grid) {
  const R = grid.length;
  const C = grid[0].length;
  let result = 0;

  /** @type {number[][]} */
  const dp = [];
  for (let i = 0; i < R; i += 1) {
    dp.push([1, 0]);
  }

  for (let c = 1; c < C; c += 1) {
    for (let r = 0; r < R; r += 1) {
      if (grid[r][c] > grid[r][c - 1] && dp[r][0] > 0) {
        dp[r][1] = Math.max(dp[r][1], dp[r][0] + 1);
      }

      if (r - 1 >= 0 && grid[r][c] > grid[r - 1][c - 1] && dp[r - 1][0] > 0) {
        dp[r][1] = Math.max(dp[r][1], dp[r - 1][0] + 1);
      }

      if (r + 1 < R && grid[r][c] > grid[r + 1][c - 1] && dp[r + 1][0] > 0) {
        dp[r][1] = Math.max(dp[r][1], dp[r + 1][0] + 1);
      }

      result = Math.max(result, dp[r][1] - 1);
    }

    for (let k = 0; k < R; k += 1) {
      dp[k][0] = dp[k][1];
      dp[k][1] = 0;
    }
  }
  return result;
}

function main() {
  const inputs = [
    [
      [2, 4, 3, 5],
      [5, 4, 9, 3],
      [3, 4, 2, 11],
      [10, 9, 13, 15],
    ],
    [
      [3, 2, 4],
      [2, 1, 9],
      [1, 1, 7],
    ],
  ];

  for (const grid of inputs) {
    const result = maxMoves(grid);
    console.log(result);
  }
}
main();
