/**
 * @param {string} s
 * @return {string}
 */
var maskPII = function(s) {
  const atIndex = s.indexOf("@");
  if (atIndex >= 0) {
    return (s.substring(0, 1) + "*****" + s.substring(atIndex - 1)).toLowerCase();
  } else {
    const digits = s.replaceAll(/\D+/g, "");
    const local = "***-***-" + digits.substring(digits.length - 4);
    if (digits.length === 10) {
      return local;
    }

    let result = "+";
    for (let i = 0; i < digits.length - 10; ++i) {
      result += "*";
    }
    return result + "-" + local;
  }
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
      s: "1(234)567-890",
    },
  ];

  for (const input of inputs) {
    const result = maskPII(input.s);
    console.log(result);
  }
}
main();
