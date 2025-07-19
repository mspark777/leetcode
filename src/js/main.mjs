/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string} address
 * @return {string}
 */
var defangIPaddr = function (address) {
  return address.split(".").join("[.]");
};

function main() {
  const inputs = ["1.1.1.1", "255.100.50.0"];

  for (const input of inputs) {
    const result = defangIPaddr(input);
    console.log(result);
  }
}
main();
