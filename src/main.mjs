/**
 * @param {number} n
 * @returns {number}
 */
function concatenatedBinary (n) {
  let result = 1
  let len = 4
  const mod = 10 ** 9 + 7

  for (let i = 2; i <= n; i += 1) {
    if (i === len) {
      len *= 2
    }

    result = ((result * len) + i) % mod
  }

  return result
}

async function main () {
  const inputs = [
    {
      n: 1
    },
    {
      n: 3
    },
    {
      n: 12
    }
  ]

  for (const { n } of inputs) {
    const result = concatenatedBinary(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
