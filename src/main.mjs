/**
 * @param {number} n
 * @return {boolean}
 */
function isPowerOfTwo (n) {
  const b = BigInt(n)
  return b <= 0 ? false : (b & (b - 1n)) === 0n
}

async function main () {
  const inputs = [
    {
      n: 1
    },
    {
      n: 16
    },
    {
      n: 3
    }
  ]

  for (const { n } of inputs) {
    const result = isPowerOfTwo(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
