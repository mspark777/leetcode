/**
 * @param {number} n
 * @returns {Generator<number>}
 */
function* range(n) {
  let i = 1 - n;
  while (i < n) {
    yield i;
    i += 2;
  }
}

/**
 * @param {number} n
 * @return {number[]}
 */
var sumZero = function (n) {
  return [...range(n)];
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
      n: 5,
    },
    {
      n: 3,
    },
    {
      n: 1,
    },
  ];

  for (const input of inputs) {
    const result = sumZero(input.n);
    console.log(result);
  }
}
main();
