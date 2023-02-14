/**
  * @param {string} a
  * @param {string} b
  * @returns {string}
  */
function addBinary (a, b) {
  const result = BigInt(`0b${a}`) + BigInt(`0b${b}`)
  return result.toString(2)
}

async function main () {
  const inputs = [
    ['11', '1'],
    ['1010', '1011']
  ]

  for (const [a, b] of inputs) {
    const result = addBinary(a, b)
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
