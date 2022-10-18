/**
 * @param {number} n
 * @returns {string}
*/
function countAndSay (n) {
  let result = ['1']
  for (let i = 1; i < n; i += 1) {
    const temp = []
    let count = 1
    let ch = result[0]
    for (let j = 1; j < result.length; j += 1) {
      const c = result[j]
      if (ch === c) {
        count += 1
      } else {
        temp.push(count.toString())
        temp.push(ch)

        ch = c
        count = 1
      }
    }
    temp.push(count.toString())
    temp.push(ch)
    result = temp
  }

  return result.join('')
}

async function main () {
  const inputs = [
    1, 4
  ]

  for (const n of inputs) {
    const result = countAndSay(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
