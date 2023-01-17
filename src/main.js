/**
 * @param {string} s
 * @returns {number}
 */
function minFlipsMonoIncr (s) {
  let result = 0
  let num = 0

  for (const c of s) {
    if (c === '0') {
      result = Math.min(num, result + 1)
    } else {
      num += 1
    }
  }

  return result
}

async function main () {
  const inputs = [
    '00110',
    '010110',
    '00011000'
  ]

  for (const s of inputs) {
    const result = minFlipsMonoIncr(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
