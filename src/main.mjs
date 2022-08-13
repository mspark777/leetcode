/**
 * @param {string} s
 * @param {string[]} words
 * @return {number[]}
 */
function findSubstring (s, words) {
  const wordsLen = words.length
  const wordLen = words[0].length

  const result = []
  const wordCount = new Map()
  for (const word of words) {
    const cnt = wordCount.get(word) ?? 0
    wordCount.set(word, cnt + 1)
  }

  const lastWindowIndex = s.length - (wordsLen * wordLen)
  for (let i = 0; i <= lastWindowIndex; i += 1) {
    const twordCount = new Map(wordCount)

    for (let j = i; (j < s.length) && (twordCount.size > 0); j += wordLen) {
      const str = s.substring(j, j + wordLen)
      const cnt = twordCount.get(str) ?? 0
      if (cnt === 0) {
        break
      } else {
        if (cnt === 1) {
          twordCount.delete(str)
        } else {
          twordCount.set(str, cnt - 1)
        }
      }
    }

    if (twordCount.size === 0) {
      result.push(i)
    }
  }

  return result
}

async function main () {
  const inputs = [
    {
      s: 'barfoothefoobarman',
      words: ['foo', 'bar']
    },
    {
      s: 'wordgoodgoodgoodbestword',
      words: ['word', 'good', 'best', 'word']
    },
    {
      s: 'barfoofoobarthefoobarman',
      words: ['bar', 'foo', 'the']
    }
  ]

  for (const { s, words } of inputs) {
    const result = findSubstring(s, words)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
