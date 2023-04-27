/**
  * @param {number} n
  * @returns {number}
  */
function bulbSwitch (n) {
  return Math.floor(Math.sqrt(n))
}

async function main () {
  const inputs = [
    3, 0, 1
  ]

  for (const n of inputs) {
    const result = bulbSwitch(n)
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
