/**
 * @param {string} sequence
 * @param {string} word
 * @return {number}
 */
var maxRepeating = function (sequence, word) {
  let result = 0;
  const words = [word];

  while (sequence.includes(words.join(""))) {
    result += 1;
    words.push(word);
  }

  return result;
};

/**
 * @typedef Input
 * @property {string} sequence
 * @property {string} word
 */

/**
 * @return {undefined}
 */
function main() {
  /** @type Input[] */
  const inputs = [
    {
      sequence: "ababc",
      word: "ab",
    },
    {
      sequence: "ababc",
      word: "ba",
    },
    {
      sequence: "ababc",
      word: "ac",
    },
  ];

  for (const input of inputs) {
    const result = maxRepeating(input.sequence, input.word);
    console.log(result);
  }
}
main();
