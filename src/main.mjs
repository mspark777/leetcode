/**
 * @param {number} n
 * @return {boolean}
 */
function next (n) {
  let result = 0n
  for (let i = BigInt(n); i > 0n; i /= 10n) {
    const d = i % 10n
    result += d * d
  }

  return Number(result)
}

/**
 * @param {number} n
 * @return {boolean}
 */
function isHappy (n) {
  let slow = n
  let fast = next(n)
  while ((fast !== 1) && (slow !== fast)) {
    slow = next(slow)
    fast = next(next(fast))
  }

  return fast === 1
}

async function main () {
  const inputs = [
    {
      n: 19
    },
    {
      n: 2
    }
  ]

  for (const { n } of inputs) {
    const result = isHappy(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
