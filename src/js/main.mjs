/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string[]} words
 * @param {string} pref
 * @return {number}
 */
function prefixCount(words, pref) {
  let result = 0;

  for (const word of words) {
    const sub = word.slice(0, pref.length);
    if (sub === pref) {
      result += 1;
    }
  }

  return result;
}

function main() {
  const inputs = [
    [["pay", "attention", "practice", "attend"], "at"],
    [["leetcode", "win", "loops", "success"], "code"],
  ];

  for (const [words, pref] of inputs) {
    const result = prefixCount(words, pref);
    console.log(result);
  }
}
main();
