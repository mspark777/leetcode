/**
 * @param {string} s
 * @param {string} t
 * @returns {boolean}
 */
function isSubsequence (s, t) {
  let si = 0

  for (let i = 0; i < t.length; i += 1) {
    if (s.charCodeAt(si) === t.charCodeAt(i)) {
      si += 1
    }

    if (s.length === si) {
      return true
    }
  }

  return s.length < 1
}

async function main () {
  const inputs = [
    ['abc', 'ahbgdc'],
    ['axc', 'ahbgdc']
  ]

  for (const [s, t] of inputs) {
    const result = isSubsequence(s, t)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
