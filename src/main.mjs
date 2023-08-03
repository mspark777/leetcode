/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {string} digits
  * @returns {string[]}
  */
function letterCombinations (digits) {
  if (digits.length < 1) {
    return []
  }

  /** @type {Map<string, string>} */
  const lettersMap = new Map()
  lettersMap.set('2', 'abc')
  lettersMap.set('3', 'def')
  lettersMap.set('4', 'ghi')
  lettersMap.set('5', 'jkl')
  lettersMap.set('6', 'mno')
  lettersMap.set('7', 'pqrs')
  lettersMap.set('8', 'tuv')
  lettersMap.set('9', 'wxyz')

  /** @type {string[]} */
  const stack = ['']
  /** @type {string[]} */
  const result = []

  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const toplen = top.length
    const ch = digits[toplen]
    const letters = lettersMap.get(ch) ?? ''
    for (const letter of letters) {
      const combination = top + letter
      if (combination.length === digits.length) {
        result.push(combination)
      } else {
        stack.push(combination)
      }
    }
  }

  return result
}

function main () {
  const inputs = [
    '23',
    '',
    '2'
  ]

  for (const digits of inputs) {
    const result = letterCombinations(digits)
    console.log(result)
  }
}
main()
