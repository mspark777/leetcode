/**
 * @param {string[]} words
 * @returns {string[]}
 */
function findAllConcatenatedWordsInADict (words) {
  const dictionary = new Set(words)
  const result = []
  for (const word of words) {
    const wlen = word.length
    const dp = new Array(wlen + 1).fill(false)
    dp[0] = true

    for (let i = 1; i <= wlen; i += 1) {
      for (let j = i === wlen ? 1 : 0; !dp[i] && (j < i); j += 1) {
        dp[i] = dp[j] && dictionary.has(word.substring(j, i))
      }
    }

    if (dp[wlen]) {
      result.push(word)
    }
  }

  return result
}

async function main () {
  const inputs = [
    ['cat', 'cats', 'catsdogcats', 'dog', 'dogcatsdog', 'hippopotamuses', 'rat', 'ratcatdogcat'],
    ['cat', 'dog', 'catdog']
  ]

  for (const words of inputs) {
    const result = findAllConcatenatedWordsInADict(words)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
