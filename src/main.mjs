/** @type {(n: number) => number} */
let guess = n => n

/**
 * @param {number} pick
 * @returns {(n: number) => number}
*/
function Guess (pick) {
  return n => {
    if (n < pick) {
      return 1
    } else if (n > pick) {
      return -1
    } else {
      return 0
    }
  }
}

/**
 * @param {number} n
 * @returns {number}
*/
function guessNumber (n) {
  let left = 1
  let right = n
  while (left <= right) {
    const m = Math.round((left + right) / 2)
    const res = guess(m)
    if (res < 0) {
      right = m - 1
    } else if (res > 0) {
      left = m + 1
    } else {
      return m
    }
  }

  return -1
}

async function main () {
  const inputs = [[10, 6], [1, 1], [2, 1]]

  for (const [n, pick] of inputs) {
    guess = Guess(pick)
    const result = guessNumber(n)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
