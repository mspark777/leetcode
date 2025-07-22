/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string} s
 * @return {number}
 */
var balancedStringSplit = function (s) {
  let result = 0;
  let balance = 0;

  for (let c of s) {
    if (c === "L") {
      balance -= 1;
    } else if (c === "R") {
      balance += 1;
    }

    if (balance === 0) {
      result += 1;
    }
  }

  return result;
};

/**
 * @typedef Input
 * @property {string} s
 */

/**
 * @return {undefined}
 */
function main() {
  /** @type Input[] */
  const inputs = [
    {
      s: "RLRRLLRLRL",
    },
    {
      s: "RLRRRLLRLL",
    },
    {
      s: "LLLLRRRR",
    },
  ];

  for (const input of inputs) {
    const result = balancedStringSplit(input.s);
    console.log(result);
  }
}
main();
