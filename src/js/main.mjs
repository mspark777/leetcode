/**
 * @param {string} s
 * @return {boolean}
 */
var checkOnesSegment = function (s) {
  return s.indexOf("01") === -1;
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
      s: "1001",
    },
    {
      s: "110",
    },
  ];

  for (const input of inputs) {
    const result = checkOnesSegment(input.s);
    console.log(result);
  }
}
main();
