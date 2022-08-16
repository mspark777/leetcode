/**
 * @param {string} s
 * @return {number}
 */
function firstUniqChar (s) {
  const memo = new Map()

  for (const ch of s) {
    const cnt = memo.get(ch) ?? 0
    memo.set(ch, cnt + 1)
  }

  for (let i = 0; i < s.length; i += 1) {
    const ch = s.charAt(i)
    if (memo.get(ch) === 1) {
      return i
    }
  }

  return -1
}

async function main () {
  const inputs = [
    {
      s: 'leetcode'
    },
    {
      s: 'loveleetcode'
    },
    {
      s: 'aabb'
    }
  ]

  for (const { s } of inputs) {
    const result = firstUniqChar(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
