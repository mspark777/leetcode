/**
 * @param {string} columnTitle
 * @return {number}
 */
function titleToNumber (columnTitle) {
  const factor = 'A'.charCodeAt(0) - 1
  let result = 0
  for (let i = 0; i < columnTitle.length; i += 1) {
    const code = columnTitle.charCodeAt(i)
    result = result * 26 + (code - factor)
  }

  return result
}

async function main () {
  const inputs = [
    {
      columnTitle: 'A'
    },
    {
      columnTitle: 'AB'
    },
    {
      columnTitle: 'ZY'
    }
  ]

  for (const { columnTitle } of inputs) {
    const result = titleToNumber(columnTitle)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
