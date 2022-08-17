/**
 * @param {string[]} words
 * @return {number}
 */
function uniqueMorseRepresentations (words) {
  const codes = [
    '.-', '-...', '-.-.', '-..', '.', '..-.', '--.', '....', '..', '.---', '-.-',
    '.-..', '--', '-.', '---', '.--.', '--.-', '.-.', '...', '-', '..-', '...-',
    '.--', '-..-', '-.--', '--..'
  ]
  const acode = 'a'.charCodeAt(0)
  const seen = new Set()
  for (const word of words) {
    const wcodes = []
    for (let i = 0; i < word.length; i += 1) {
      const j = word.charCodeAt(i) - acode
      wcodes.push(codes[j])
    }

    seen.add(wcodes.join(''))
  }

  return seen.size
}

async function main () {
  const inputs = [
    {
      words: ['gin', 'zen', 'gig', 'msg']
    },
    {
      words: ['a']
    }
  ]

  for (const { words } of inputs) {
    const result = uniqueMorseRepresentations(words)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
