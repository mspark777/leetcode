/**
 * @param {string} s
 * @param {string} t
 * @returns {string}
 */
function findTheDifference (s, t) {
  let sum = 0
  for (let i = 0; i < t.length; i += 1) {
    sum += t.charCodeAt(i)
  }

  for (let i = 0; i < s.length; i += 1) {
    sum -= s.charCodeAt(i)
  }

  return String.fromCharCode(sum)
}

async function main () {
  const inputs = [
    ['abcd', 'abcde'],
    ['', 'y']
  ]

  for (const [s, t] of inputs) {
    const result = findTheDifference(s, t)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
