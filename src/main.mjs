/**
 * @param {string[]} arr
 * @returns {number}
*/
function maxLength (arr) {
  const dp = [new Set()]
  for (const str of arr) {
    const memo = new Set(str)
    if (memo.size < str.length) {
      continue
    }

    for (const d of [...dp]) {
      if ([...d].some(x => memo.has(x))) {
        continue
      } else {
        dp.push(new Set([...d, ...memo]))
      }
    }
  }

  return Math.max(...[...dp].map(s => s.size))
}

async function main () {
  const inputs = [
    ['un', 'iq', 'ue'],
    ['cha', 'r', 'act', 'ers'],
    ['abcdefghijklmnopqrstuvwxyz']
  ]

  for (const arr of inputs) {
    const result = maxLength(arr)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
