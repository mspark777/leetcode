/**
  * @param {string} word1
  * @param {string} worl2
  */
function mergeAlternately (word1, word2) {
  const rlen = word1.length + word2.length
  const result = new Array(rlen)
  let pos = 0
  for (let i = 0; i < Math.max(word1.length, word2.length); i += 1) {
    const ch1 = word1.charAt(i)
    const ch2 = word2.charAt(i)

    if (ch1 != null) {
      result[pos] = ch1
      pos += 1
    }

    if (ch2 != null) {
      result[pos] = ch2
      pos += 1
    }
  }

  return result.join('')
}

async function main () {
  const inputs = [
    ['abc', 'pqr'],
    ['ab', 'pqrs'],
    ['abcd', 'pq']
  ]

  for (const [word1, word2] of inputs) {
    const result = mergeAlternately(word1, word2)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
