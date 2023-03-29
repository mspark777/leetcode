/**
  * @param {number[]} satisfaction
  * @returns {number}
  */
function maxSatisfaction (satisfaction) {
  satisfaction.sort((a, b) => b - a)

  let maxSatisfaction = 0
  let suffixSum = 0

  for (const s of satisfaction) {
    suffixSum += s
    if (suffixSum <= 0) {
      break
    }
    maxSatisfaction += suffixSum
  }

  return maxSatisfaction
}

async function main () {
  const inputs = [
    [-1, -8, 0, 5, -9],
    [4, 3, 2],
    [-1, -4, -5]
  ]

  for (const satisfaction of inputs) {
    const result = maxSatisfaction(satisfaction)
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
