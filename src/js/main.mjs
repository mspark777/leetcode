/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} bits
 * @param {number} i
 * @param {number} value
 * @returns {undefined}
 */
function updateBits(bits, i, value) {
  while (i < bits.length) {
    bits[i] += value;
    i += i & -i;
  }
}

/**
 * @param {number[]} bits
 * @param {number} i
 * @returns {number}
 */
function getPrefixSum(bits, i) {
  let sum = 0;
  while (i > 0) {
    sum += bits[i];
    i -= i & -i;
  }
  return sum;
}

/**
 * @param {number[]} rating
 * @return {number}
 */
function numTeams(rating) {
  const maxRating = Math.max(...rating);
  const leftBits = [];
  const rightBits = [];
  for (let i = 0; i <= maxRating; i += 1) {
    leftBits.push(0);
    rightBits.push(0);
  }

  for (const r of rating) {
    updateBits(rightBits, r, 1);
  }

  let result = 0;
  for (const r of rating) {
    updateBits(rightBits, r, -1);

    const smallerRatingLeft = getPrefixSum(leftBits, r - 1);
    const smallerRatingRight = getPrefixSum(rightBits, r - 1);
    const largerRatingLeft =
      getPrefixSum(leftBits, maxRating) - getPrefixSum(leftBits, r);
    const largerRatingRight =
      getPrefixSum(rightBits, maxRating) - getPrefixSum(rightBits, r);

    result += smallerRatingLeft * largerRatingRight;
    result += largerRatingLeft * smallerRatingRight;
    updateBits(leftBits, r, 1);
  }

  return result;
}

function main() {
  const inputs = [
    [2, 5, 3, 4, 1],
    [2, 1, 3],
    [1, 2, 3, 4],
  ];

  for (const rating of inputs) {
    const result = numTeams(rating);
    console.log(result);
  }
}
main();
