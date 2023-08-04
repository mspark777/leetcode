/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {string} s
  * @param {string[]} wordDict
  * @returns {boolean}
  */
function wordBreak (s, wordDict) {
  const words = new Set(wordDict)
  const checks = new Array(s.length + 1).fill(false)
  checks[0] = true

  for (let right = 1; right <= s.length; right += 1) {
    for (let left = 0; left < right; left += 1) {
      if (checks[left] !== true) {
        continue
      }

      const sub = s.substring(left, right)
      if (words.has(sub)) {
        checks[right] = true
        break
      }
    }
  }

  return checks[s.length]
}

function main () {
  const inputs = [
    { s: 'leetcode', wordDict: ['leet', 'code'] },
    { s: 'applepenapple', wordDict: ['apple', 'pen'] },
    { s: 'catsandog', wordDict: ['cats', 'dog', 'sand', 'and', 'cat'] }
  ]

  for (const { s, wordDict } of inputs) {
    const result = wordBreak(s, wordDict)
    console.log(result)
  }
}
main()
