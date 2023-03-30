/**
  * @param {string} s1
  * @param {string} s2
  * @returns {boolean}
  */
function isScramble (s1, s2) {
  const dp = new Array(s1.length + 1)
  for (let i = 0; i <= s1.length; i += 1) {
    dp[i] = new Array(s1.length)
    for (let j = 0; j < s1.length; j += 1) {
      dp[i][j] = new Array(s1.length).fill(false)
    }
  }

  for (let i = 0; i < s1.length; i += 1) {
    for (let j = 0; j < s1.length; j += 1) {
      dp[1][i][j] = s1[i] === s2[j]
    }
  }

  for (let length = 2; length <= s1.length; length += 1) {
    for (let i = 0; i <= (s1.length - length); i += 1) {
      for (let j = 0; j <= (s1.length - length); j += 1) {
        for (let newLength = 1; newLength < length; newLength += 1) {
          const dp1 = dp[newLength][i]
          const dp2 = dp[length - newLength][i + newLength]

          dp[length][i][j] ||= dp1[j] && dp2[j + newLength]
          dp[length][i][j] ||= dp1[j + length - newLength] && dp2[j]
        }
      }
    }
  }

  return dp[s1.length][0][0]
}

async function main () {
  const inputs = [
    ['great', 'rgeat'],
    ['abcde', 'caebd'],
    ['a', 'a'],
    ['aa', 'ab']
  ]

  for (const [s1, s2] of inputs) {
    const result = isScramble(s1, s2)
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
