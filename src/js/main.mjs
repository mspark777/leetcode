/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {bigint} target
 * @param {bigint} maxDoubles
 * @return {bigint}
 */
function solve(target, maxDoubles) {
  let moves = 0n;
  while (target > 1n && maxDoubles > 0n) {
    if (target % 2n === 1n) {
      moves += 2n;
    } else {
      moves += 1n;
    }

    maxDoubles -= 1n;
    target /= 2n;
  }

  return moves + target - 1n;
}

/**
 * @param {number} target
 * @param {number} maxDoubles
 * @return {number}
 */
function minMoves(target, maxDoubles) {
  const moves = solve(BigInt(target), BigInt(maxDoubles));
  return Number(moves);
}

function main() {
  const inputs = [
    [5, 0],
    [19, 2],
    [10, 4],
  ];

  for (const [target, maxDouble] of inputs) {
    const result = minMoves(target, maxDouble);
    console.log(result);
  }
}
main();
