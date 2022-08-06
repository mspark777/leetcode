/**
 * @param {number} buckets
 * @param {number} minutesToDie
 * @param {number} minutesToTest
 * @return {number}
 */
function poorPigs (buckets, minutesToDie, minutesToTest) {
  return Math.ceil(Math.log(buckets) / Math.log(minutesToTest / minutesToDie + 1))
}

async function main () {
  const inputs = [
    {
      buckets: 1000,
      minutesToDie: 15,
      minutesToTest: 60
    },
    {
      buckets: 4,
      minutesToDie: 15,
      minutesToTest: 15
    },
    {
      buckets: 4,
      minutesToDie: 15,
      minutesToTest: 30
    }
  ]

  for (const { buckets, minutesToDie, minutesToTest } of inputs) {
    const result = poorPigs(buckets, minutesToDie, minutesToTest)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
