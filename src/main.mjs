/**
 * @param {string} s
 * @returns {boolean}
 */
function halvesAreAlike (s) {
  const vowels = new Set(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'])
  let first = 0
  let second = 0

  for (let i = 0, j = Math.round(s.length / 2); j < s.length; i += 1, j += 1) {
    if (vowels.has(s[i])) {
      first += 1
    }

    if (vowels.has(s[j])) {
      second += 1
    }
  }

  return first === second
}

async function main () {
  const inputs = [
    'book',
    'textbook'
  ]

  for (const s of inputs) {
    const result = halvesAreAlike(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
