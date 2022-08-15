/**
 * @param {number} columnNumber
 * @return {string}
 */
function convertToTitle (columnNumber) {
  let n = BigInt(columnNumber)
  const result = []
  const ACODE = 'A'.charCodeAt(0)
  const SIZE = 26n

  while (n > 0n) {
    n -= 1n

    const temp = ACODE + Number((n % SIZE))
    result.push(temp)

    n /= 26n
  }

  return String.fromCharCode(...result.reverse())
}

async function main () {
  const inputs = [
    {
      columnNumber: 1
    },
    {
      columnNumber: 28
    },
    {
      columnNumber: 701
    }
  ]

  for (const { columnNumber } of inputs) {
    const result = convertToTitle(columnNumber)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
