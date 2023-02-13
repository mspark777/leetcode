/**
  * @param {number} low
  * @param {number} high
  * @returns {number}
  */
function countOdds (low, high) {
  let l = BigInt(low)
  const h = BigInt(high)
  if ((l & 1n) === 0n) {
    l += 1n
  }

  return l > h ? 0 : Number(((h - l) / 2n) + 1n)
}

async function main () {
  const inputs = [
    [3, 7],
    [8, 10]
  ]

  for (const [low, high] of inputs) {
    const result = countOdds(low, high)
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
