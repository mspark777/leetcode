/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable @typescript-eslint/no-extraneous-class */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/no-unnecessary-condition */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string} s1
 * @param {string} s2
 * @return {string[]}
 */
function uncommonFromSentences(s1, s2) {
  const counts = new Map();
  for (const word of s1.split(" ")) {
    const count = counts.get(word) ?? 0;
    counts.set(word, count + 1);
  }
  for (const word of s2.split(" ")) {
    const count = counts.get(word) ?? 0;
    counts.set(word, count + 1);
  }

  const result = [];
  for (const [word, count] of counts) {
    if (count === 1) {
      result.push(word);
    }
  }

  return result;
}

/*
class Solution(object):
    def uncommonFromSentences(self, A, B):
        count = {}
        for word in A.split():
            count[word] = count.get(word, 0) + 1
        for word in B.split():
            count[word] = count.get(word, 0) + 1

        #Alternatively:
        #count = collections.Counter(A.split())
        #count += collections.Counter(B.split())

        return [word for word in count if count[word] == 1]
 */

function main() {
  const inputs = [
    ["this apple is sweet", "this apple is sour"],
    ["apple apple", "banana"],
  ];

  for (const [s1, s2] of inputs) {
    const result = uncommonFromSentences(s1, s2);
    console.log(result);
  }
}
main();
