/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string} s
 * @return {string}
 */
function reverseParentheses(s) {
  const chars = s.split("");
  const indices = [];
  const pair = new Array(chars.length).fill(0);

  for (const [i, ch] of chars.entries()) {
    if (ch === "(") {
      indices.push(i);
    } else if (ch === ")") {
      const j = indices.pop();
      pair[i] = j;
      pair[j] = i;
    }
  }

  const result = [];
  let direction = 1;
  for (let curr = 0; curr < s.length; curr += direction) {
    const ch = chars[curr];
    if (["(", ")"].includes(ch)) {
      curr = pair[curr];
      direction = -direction;
    } else {
      result.push(ch);
    }
  }

  return result.join("");
}

function main() {
  const inputs = ["(abcd)", "(u(love)i)", "(ed(et(oc))el)"];

  for (const s of inputs) {
    const result = reverseParentheses(s);
    console.log(result);
  }
}
main();
