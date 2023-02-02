/**
  * @param {string[]} words
  * @param {string} order
  * @returns {boolean}
  */
function isAlienSorted (words, order) {
  const ACODE = 'a'.charCodeAt(0)
  const orders = new Array(26)
  for (let i = 0; i < order.length; i += 1) {
    const code = order.charCodeAt(i) - ACODE
    orders[code] = i
  }

  for (let i = 0; i < words.length - 1; i += 1) {
    const current = words[i]
    const next = words[i + 1]

    for (let j = 0; j < current.length; j += 1) {
      if (j >= next.length) {
        return false
      }

      const curCh = current.charCodeAt(j) - ACODE
      const nxtCh = next.charCodeAt(j) - ACODE
      const curOrd = orders[curCh]
      const nxtOrd = orders[nxtCh]
      if (curOrd !== nxtOrd) {
        if (curOrd > nxtOrd) {
          return false
        } else {
          break
        }
      }
    }
  }

  return true
}

async function main () {
  const inputs = [
    { words: ['hello', 'leetcode'], order: 'hlabcdefgijkmnopqrstuvwxyz' },
    { words: ['word', 'world', 'row'], order: 'worldabcefghijkmnpqstuvxyz' },
    { words: ['apple', 'app'], order: 'abcdefghijklmnopqrstuvwxyz' },
    { words: ['kuvp', 'q'], order: 'ngxlkthsjuoqcpavbfdermiywz' }
  ]

  for (const { words, order } of inputs) {
    const result = isAlienSorted(words, order)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
