/**
  * @param {number[]} days
  * @param {number[]} costs
  * @param {number[]} memos
  * @param {number[]} durations
  * @param {number} i
  * @returns {number}
  */
function dp (days, costs, memos, durations, i) {
  if (i >= days.length) {
    return 0
  } else if (memos[i] !== 0) {
    return memos[i]
  }

  let result = Number.MAX_SAFE_INTEGER
  let j = i
  for (const [d, duration] of durations.entries()) {
    while (j < days.length) {
      const k = days[i] + duration
      if (days[j] < k) {
        j += 1
      } else {
        break
      }
    }

    const recv = dp(days, costs, memos, durations, j)
    result = Math.min(result, recv + costs[d])
  }

  memos[i] = result
  return result
}

/**
  * @param {number[]} days
  * @param {number[]} costs
  * @returns {number}
  */
function mincostTickets (days, costs) {
  const memos = new Array(days.length).fill(0)
  const durations = [1, 7, 30]

  return dp(days, costs, memos, durations, 0)
}

async function main () {
  const inputs = [
    [[1, 4, 6, 7, 8, 20], [2, 7, 15]],
    [[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], [2, 7, 15]]
  ]

  for (const [days, costs] of inputs) {
    const result = mincostTickets(days, costs)
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
