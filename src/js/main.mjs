/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string} sentence1
 * @param {string} sentence2
 * @return {boolean}
 */
function areSentencesSimilar(sentence1, sentence2) {
  const words1 = sentence1.split(" ");
  const words2 = sentence2.split(" ");
  let ends1 = words1.length - 1;
  let ends2 = words2.length - 1;

  if (ends1 > ends2) {
    return areSentencesSimilar(sentence2, sentence1);
  }

  let start = 0;
  while (start < words1.length && words1[start] === words2[start]) {
    start += 1;
  }

  while (ends1 >= 0 && words1[ends1] === words2[ends2]) {
    ends1 -= 1;
    ends2 -= 1;
  }

  return ends1 < start;
}

function main() {
  const inputs = [
    ["My name is Haley", "My Haley"],
    ["of", "A lot of words"],
    ["Eating right now", "Eating"],
  ];

  for (const [s1, s2] of inputs) {
    const result = areSentencesSimilar(s1, s2);
    console.log(result);
  }
}
main();
