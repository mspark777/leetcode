/**
 * @param {number} n
 * @return {boolean}
 */
function isPowerOfFour (n) {
  const i = BigInt(n)
  return i > 0n && ((i & (i - 1n)) === 0n) && ((i & BigInt(0x55555555)) !== 0n)
}

async function main () {
  const inputs = [
    {
      n: 16
    },
    {
      n: 5
    },
    {
      n: 1
    }
  ]

  for (const { n } of inputs) {
    const result = isPowerOfFour(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
