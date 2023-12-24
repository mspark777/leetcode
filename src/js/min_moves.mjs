/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * Usually, when finding the shortest distance,
 * it is helpful to proceed from the destination to the starting (Reversely).
 * This problem it is.
 * @param {bigint} target
 * @param {bigint} maxDoubles
 * @return {bigint}
 */
function solve(target, maxDoubles) {
  let moves = 0n;
  while (target > 1n && maxDoubles > 0n) {
    if (target % 2n === 1n) {
      // if target is odd, Need increment and dobule.
      // Example: 17 = 8 * 2 + 1
      moves += 2n;
    } else {
      moves += 1n;
    }

    maxDoubles -= 1n;
    target /= 2n;
  }

  // If no more double move, Need increment move (target - 1) times.
  return moves + target - 1n;
}

/**
 * @param {number} target
 * @param {number} maxDoubles
 * @return {number}
 */
function minMoves(target, maxDoubles) {
  // If need div operation with integers, I think that I should use bigint.
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
