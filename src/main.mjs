/**
 * @param {bigint} n
 * @returns {bigint[]}
*/
function getCounts (n) {
  const result = new Array(10).fill(0n)
  while (n > 0n) {
    const idx = Number(n % 10n)
    result[idx] += 1n
    n /= 10n
  }

  return result
}

/**
 * @param {bigint[]} c1
 * @param {bigint[]} c2
 * @returns {boolean}
*/
function compareCounts (c1, c2) {
  for (let i = 0; i < c1.length; i += 1) {
    if (c1[i] !== c2[i]) {
      return false
    }
  }

  return true
}

/**
 * @param {number} n
 * @return {boolean}
 */
function reorderedPowerOf2 (n) {
  const counts = getCounts(BigInt(n))
  for (let i = 0n; i < 31n; i += 1n) {
    if (compareCounts(counts, getCounts(1n << i))) {
      return true
    }
  }

  return false
}

async function main () {
  const inputs = [
    {
      n: 1
    },
    {
      n: 10
    },
    {
      n: 46
    }
  ]

  for (const { n } of inputs) {
    const result = reorderedPowerOf2(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
