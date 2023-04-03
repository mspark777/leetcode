/**
  * @param {number[]} people
  * @param {number} limit
  * @returns {number}
  */
function numRescueBoats (people, limit) {
  people.sort((a, b) => a - b)
  let left = 0
  let right = people.length - 1

  let result = 0
  while (left <= right) {
    result += 1
    const light = people[left]
    const heavy = people[right]
    const total = light + heavy

    right -= 1
    if (total <= limit) {
      left += 1
    }
  }

  return result
}

async function main () {
  const inputs = [
    [[1, 2], 3],
    [[3, 2, 2, 1], 3],
    [[3, 5, 3, 4], 5]
  ]

  for (const [people, limit] of inputs) {
    const result = numRescueBoats(people, limit)
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
