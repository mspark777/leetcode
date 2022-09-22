/**
 * @param {string} s
 * @returns {string}
 */
function reverseWords (s) {
  return s.split(' ').map(ss => ss.split('').reverse().join('')).join(' ')
}

async function main () {
  const inputs = [
    {
      s: "Let's take LeetCode contest"
    },
    {
      s: 'God Ding'
    }
  ]

  for (const { s } of inputs) {
    const result = reverseWords(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
