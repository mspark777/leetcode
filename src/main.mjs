/**
 * @param {number} n - a positive integer
 * @return {number} - a positive integer
 */
function reverseBits (n) {
  let i = BigInt(n)
  i = ((i & BigInt(0xffff0000)) >> 16n) | ((i & BigInt(0x0000ffff)) << 16n)
  i = ((i & BigInt(0xff00ff00)) >> 8n) | ((i & BigInt(0x00ff00ff)) << 8n)
  i = ((i & BigInt(0xf0f0f0f0)) >> 4n) | ((i & BigInt(0x0f0f0f0f)) << 4n)
  i = ((i & BigInt(0xcccccccc)) >> 2n) | ((i & BigInt(0x33333333)) << 2n)
  i = ((i & BigInt(0xaaaaaaaa)) >> 1n) | ((i & BigInt(0x55555555)) << 1n)

  return Number(i)
}

async function main () {
  const inputs = [
    {
      n: 43261596
    },
    {
      n: 4294967293
    }
  ]

  for (const { n } of inputs) {
    const result = reverseBits(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
