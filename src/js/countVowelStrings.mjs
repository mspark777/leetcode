/**
 * @param {number} n
 * @return {number}
 */
export const countVowelStrings = function (n) {
  return (n + 1) * (n + 2) * (n + 3) * (n + 4) / 24
}
