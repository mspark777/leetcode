/**
 * @param {string} pattern
 * @param {string} s
 * @returns {boolean}
 */
function wordPattern(pattern, s) {
  const words = s.split(' ')
  const patterns = pattern.split('')
  if (words.length !== patterns.length) {
    return false
  }

  const ptow = new Map()
  const wtop = new Map()
  for (let i = 0; i < words.length; i += 1) {
    const word = words[i]
    const p = patterns[i]
    if (ptow.has(p)) {
      if (ptow.get(p) !== word) {
        return false
      }
    } else {
      ptow.set(p, word)
    }

    if (wtop.has(word)) {
      if (wtop.get(word) !== p) {
        return false
      }
    } else {
      wtop.set(word, p)
    }
  }

  return true
}

async function main() {
  const inputs = [
    ['abba', 'dog cat cat dog'],
    ['abba', 'dog cat cat fish'],
    ['aaaa', 'dog cat cat dog'],
    ['abba', 'dog dog dog dog']
  ]

  for (const [pattern, s] of inputs) {
    const result = wordPattern(pattern, s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
