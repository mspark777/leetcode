/**
 * @param {string} s
 * @return {string}
 */
var replaceDigits = function (s) {
  /** @type {string[]} */
  const result = [];

  let prev = "";
  for (const ch of s) {
    const chCode = ch.charCodeAt(0);
    const zeroCode = "0".charCodeAt(0);
    const nineCode = "9".charCodeAt(0);
    if (zeroCode <= chCode && chCode <= nineCode) {
      const offset = chCode - zeroCode;
      result.push(String.fromCharCode(prev.charCodeAt(0) + offset));
    } else {
      prev = ch;
      result.push(ch);
    }
  }

  return result.join("");
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
      s: "a1c1e1",
    },
    {
      s: "a1b2c3d4e",
    },
  ];

  for (const input of inputs) {
    const result = replaceDigits(input.s);
    console.log(result);
  }
}
main();
