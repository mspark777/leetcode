/**
 * @param {string} s
 * @returns {number}
 */
function numDecodings (s) {
  const ZERO = '0'.charCodeAt(0)
  if (s.charCodeAt(0) === ZERO) {
    return 0
  }

  if (s.length === 1) {
    return 1
  }

  let d1 = 1
  let d2 = 1

  for (let i = 1; i < s.length; i += 1) {
    const code1 = s.charCodeAt(i) - ZERO
    const code0 = ((s.charCodeAt(i - 1) - ZERO) * 10) + code1

    let n = 0
    if (code1 !== 0) {
      n += d1
    }

    if ((code0 >= 10) && (code0 <= 26)) {
      n += d2
    }

    d2 = d1
    d1 = n
  }

  return d1
}

async function main () {
  const inputs = [
    '12',
    '226',
    '06'
  ]

  for (const input of inputs) {
    const result = numDecodings(input)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
