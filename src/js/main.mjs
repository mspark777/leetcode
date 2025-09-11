/**
 * @param {number} n
 * @return {boolean}
 */
var isThree = function (n) {
  if (n === 1) {
    return false;
  }

  let count = 2;
  const sqrt = Math.sqrt(n);

  if (Number.isInteger(sqrt)) {
    count += 1;
  }

  for (let i = 2; i < sqrt; i += 1) {
    if (n % i === 0) {
      count += 2;
    }

    if (count > 3) {
      break;
    }
  }

  return count === 3;
};

/**
 * @typedef Input
 * @property {number} n
 */

/**
 * @return {undefined}
 */
function main() {
  /** @type Input[] */
  const inputs = [
    {
      n: 2,
    },
    {
      n: 4,
    },
  ];

  for (const input of inputs) {
    const result = isThree(input.n);
    console.log(result);
  }
}
main();
