/**
 * @param {number} num
 * @return {number}
 */
function addDigits (num) {
  return num === 0 ? 0 : 1 + (num - 1) % 9
}

async function main () {
  const inputs = [
    {
      num: 38
    },
    {
      num: 0
    }
  ]

  for (const { num } of inputs) {
    const result = addDigits(num)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
