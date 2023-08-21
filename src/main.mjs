/* eslint-disable @typescript-eslint/restrict-plus-operands */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {string} s
  * @returns {boolean}
  */
function repeatedSubstringPattern (s) {
  return s.repeat(2).slice(1, -1).includes(s)
}

function main () {
  const inputs = [
    'abab',
    'aba',
    'abcabcabcabc'
  ]

  for (const s of inputs) {
    const result = repeatedSubstringPattern(s)
    console.log(result)
  }
}
main()
