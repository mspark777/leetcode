/**
  * @param {string} s
  * @returns {string}
  */
function removeStars (s) {
  const stack = []
  for (const ch of s) {
    if (ch === '*') {
      stack.pop()
    } else {
      stack.push(ch)
    }
  }

  return stack.join('')
}

async function main () {
  const inputs = [
    'leet**cod*e',
    'erase*****'
  ]

  for (const s of inputs) {
    const result = removeStars(s)
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
