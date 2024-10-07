/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string} s
 * @return {number}
 */
function minLength(s) {
  /** @type {number[]} */
  const stack = [];

  for (const ch of s) {
    if (stack.length < 1) {
      stack.push(ch);
      continue;
    }

    if (ch === "B" && stack.at(-1) === "A") {
      stack.pop();
    } else if (ch === "D" && stack.at(-1) === "C") {
      stack.pop();
    } else {
      stack.push(ch);
    }
  }

  return stack.length;
}

function main() {
  const inputs = ["ABFCACDB", "ACBBD"];

  for (const s of inputs) {
    const result = minLength(s);
    console.log(result);
  }
}
main();
