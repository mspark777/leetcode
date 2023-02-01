/**
 * @param {number} x
 * @param {number} y
 * @returns {number}
 */
function gcd (x, y) {
  return y !== 0 ? gcd(y, x % y) : x
}

/**
 * @param {string} str1
 * @param {string} str2
 * @returns {string}
 */
function gcdOfStrings (str1, str2) {
  if (`${str1}${str2}` !== `${str2}${str1}`) {
    return ''
  }

  const gcdLen = gcd(str1.length, str2.length)
  return str1.substring(0, gcdLen)
}

async function main () {
  const inputs = [
    ['ABCABC', 'ABC'],
    ['ABABAB', 'ABAB'],
    ['LEET', 'CODE']
  ]

  for (const [str1, str2] of inputs) {
    const result = gcdOfStrings(str1, str2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
