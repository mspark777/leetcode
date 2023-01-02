/**
 * @param {string} word
 * @returns {boolean}
 */
function detectCapitalUse(word) {
  const ACODE = 'A'.charCodeAt(0)
  const ZCODE = 'Z'.charCodeAt(0)
  const inRange = code => (ACODE <= code) && (code <= ZCODE)
  let count = 0
  let begin = -1
  for (let i = 0; i < word.length; i += 1) {
    const code = word.charCodeAt(i)
    if (inRange(code)) {
      count += 1
      begin = i
    }
  }

  return (count < 1) || (count === word.length) || ((count === 1) && (begin === 0))
}

async function main() {
  const inputs = [
    "USA",
    "Google",
    "leetcode",
    "FlaG"
  ]

  for (const word of inputs) {
    const result = detectCapitalUse(word)
    console.log(result)
  }
}

await main()
