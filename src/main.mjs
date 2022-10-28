/**
 * @param {string[]} strs
 * @returns {string[][]}
 */
function groupAnagrams (strs) {
  const map = new Map()

  for (const str of strs) {
    const key = [...str].sort().join('')
    const group = map.get(key)
    if (group != null) {
      group.push(str)
    } else {
      map.set(key, [str])
    }
  }

  return Array.from(map.values())
}

async function main () {
  const inputs = [
    ['eat', 'tea', 'tan', 'ate', 'nat', 'bat'],
    [''],
    ['a']
  ]

  for (const strs of inputs) {
    const result = groupAnagrams(strs)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
