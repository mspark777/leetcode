/**
 * @param {string[][]} result
 * @param {string} s
 * @param {number} start
 * @param {string[]} current
 * @param {boolean[][]} dp
 * @returns {undefined}
 */
function dfs (result, s, start, current, dp) {
  if (start >= s.length) {
    result.push(current.slice())
  }

  for (let end = start; end < s.length; end += 1) {
    const check = (s[start] === s[end]) &&
      (((end - start) <= 2) || dp[start + 1][end - 1])
    if (check) {
      dp[start][end] = true
      current.push(s.substring(start, end + 1))
      dfs(result, s, end + 1, current, dp)
      current.pop()
    }
  }
}

/**
 * @param {string} s
 * @returns {string[][]}
 */
function partition (s) {
  const len = s.length
  const dp = Array.from(new Array(len), () => new Array(len).fill(false))
  const result = []
  const current = []
  dfs(result, s, 0, current, dp)
  return result
}

async function main () {
  const inputs = [
    'aab',
    'a'
  ]

  for (const s of inputs) {
    const result = partition(s)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
