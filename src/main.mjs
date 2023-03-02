/**
  * @param {string[]} chars
  * @returns {number}
  */
function compress (chars) {
  let newlen = 0
  for (let i = 0; i < chars.length; i += 0) {
    let groupLen = 1
    for (let j = i + groupLen; j < chars.length; j += 1) {
      if (chars[i] === chars[j]) {
        groupLen += 1
      } else {
        break
      }
    }

    chars[newlen] = chars[i]
    newlen += 1
    if (groupLen > 1) {
      for (const ch of groupLen.toString()) {
        chars[newlen] = ch
        newlen += 1
      }
    }

    i += groupLen
  }

  return newlen
}

async function main () {
  const inputs = [
    ['a', 'a', 'b', 'b', 'c', 'c', 'c'],
    ['a'],
    ['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b']
  ]

  for (const chars of inputs) {
    const result = compress(chars)
    console.log(result, chars)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
