/**
 * @param {number[]} capacity
 * @param {number[]} rocks
 * @param {number} additionalRocks
 * @returns {number}
 */
function maximumBags(capacity, rocks, additionalRocks) {
  const remains = capacity.map((c, i) => c - rocks[i])
  remains.sort((a, b) => a - b)

  let result = 0
  for (const remain of remains) {
    if (additionalRocks >= remain) {
      additionalRocks -= remain
      result += 1
    } else {
      break
    }
  }

  return result
}

async function main() {
  const inputs = [
    {
      capacity: [2, 3, 4, 5],
      rocks: [1, 2, 4, 4],
      additionalRocks: 2
    },
    {
      capacity: [10, 2, 2],
      rocks: [2, 2, 0],
      additionalRocks: 100
    }
  ]

  for (const {capacity, rocks, additionalRocks} of inputs) {
    const result = maximumBags(capacity, rocks, additionalRocks)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
