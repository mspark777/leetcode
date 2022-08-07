/**
 * @param {number} n
 * @return {number}
 */
function countVowelPermutation (n) {
  const MOD = 1000000007n

  let a = 1n
  let e = 1n
  let i = 1n
  let o = 1n
  let u = 1n

  for (let j = 1; j < n; j += 1) {
    const nexta = e + i + u
    const nexte = a + i
    const nexti = e + o
    const nexto = i
    const nextu = i + o

    a = nexta % MOD
    e = nexte % MOD
    i = nexti % MOD
    o = nexto % MOD
    u = nextu % MOD
  }

  return Number((a + e + i + o + u) % MOD)
}

async function main () {
  const inputs = [
    {
      n: 1
    },
    {
      n: 2
    },
    {
      n: 5
    },
    {
      n: 144
    }
  ]

  for (const { n } of inputs) {
    const result = countVowelPermutation(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
