/**
  * @param {number[]} num
  * @param {number} k
  * @returns {number[]}
  */
function addToArrayForm (num, k) {
  const result = []
  let cur = BigInt(k)
  for (let i = num.length - 1; i >= 0; i -= 1) {
    cur += BigInt(num[i])
    result.push(cur % 10n)
    cur /= 10n
  }

  while (cur > 0n) {
    result.push(cur % 10n)
    cur /= 10n
  }

  return result.reverse().map(b => Number(b))
}

async function main () {
  const inputs = [
    { num: [1, 2, 0, 0], k: 34 },
    { num: [2, 7, 4], k: 181 },
    { num: [2, 1, 5], k: 806 },
    { num: [1, 2, 6, 3, 0, 7, 1, 7, 1, 9, 7, 5, 6, 6, 4, 4, 0, 0, 6, 3], k: 516 },
    { num: [0], k: 10000 }
  ]

  for (const { num, k } of inputs) {
    const result = addToArrayForm(num, k)
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
