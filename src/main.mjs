/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {string} s1
  * @param {string} s2
  * @returns {number}
  */
function minimumDeleteSum (s1, s2) {
  const s1len = s1.length
  const s2len = s2.length

  if (s1len < s2len) {
    return minimumDeleteSum(s2, s1)
  }

  /** @type {number[]} */
  const curRow = new Array(s2len + 1).fill(0)
  for (let i = 1; i <= s2len; i += 1) {
    /** @type {number} */
    const prev = curRow[i - 1]
    curRow[i] = prev + s2.charCodeAt(i - 1)
  }

  for (let i = 1; i <= s1len; i += 1) {
    /** @type {number} */
    let cur = curRow[0]
    curRow[0] += s1.charCodeAt(i - 1)

    for (let j = 1; j <= s2len; j += 1) {
      let col = 0
      if (s1.charCodeAt(i - 1) === s2.charCodeAt(j - 1)) {
        col = cur
      } else {
        /** @type {number} */
        const curj = curRow[j]
        /** @type {number} */
        const prevj = curRow[j - 1]
        col = Math.min(
          s1.charCodeAt(i - 1) + curj,
          s2.charCodeAt(j - 1) + prevj
        )
      }

      /** @type {number} */
      cur = curRow[j]
      curRow[j] = col
    }
  }

  /** @type {number} */
  return curRow[s2len]
}

function main () {
  const inputs = [
    ['sea', 'eat'],
    ['delete', 'leet']
  ]

  for (const [s1, s2] of inputs) {
    const result = minimumDeleteSum(s1, s2)
    console.log(result)
  }
}
main()
