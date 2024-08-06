/* eslint-disable @typescript-eslint/no-unsafe-member-access */
/* eslint-disable @typescript-eslint/no-unsafe-call */
/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-return */
/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {string} word
 * @return {number}
 */
function minimumPushes(word) {
  const frequencyMap = new Map();

  for (const ch of word) {
    const count = frequencyMap.get(ch) ?? 0;
    frequencyMap.set(ch, count + 1);
  }

  const frequencies = [...frequencyMap.values()];
  frequencies.sort((a, b) => b - a);

  let result = 0;

  for (const [i, frequency] of frequencies.entries()) {
    const idx = BigInt(i);
    const count = (idx / 8n + 1n) * BigInt(frequency);
    result += Number(count);
  }

  return result;
}

function main() {
  const inputs = ["abcde", "xyzxyzxyzxyz", "aabbccddeeffgghhiiiiii"];

  for (const word of inputs) {
    const result = minimumPushes(word);
    console.log(result);
  }
}
main();
