/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number} n
  * @param {number[]} batteries
  * @returns {number}
  */
function maxRunTime (n, batteries) {
  const sumPower = batteries.reduce((acc, cur) => acc + cur)
  let left = 1
  let right = Math.round(sumPower / n)

  while (left < right) {
    const target = Math.round((left + right) / 2)
    const extra = batteries.reduce((acc, power) => acc + Math.min(power, target), 0)

    if (extra >= (n * target)) {
      left = target
    } else {
      right = target - 1
    }
  }

  return left
}

function main () {
  const inputs = [
    { n: 2, batteries: [3, 3, 3] },
    { n: 2, batteries: [1, 1, 1, 1] }
  ]

  for (const { n, batteries } of inputs) {
    const result = maxRunTime(n, batteries)
    console.log(result)
  }
}
main()
