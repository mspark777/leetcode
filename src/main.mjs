/**
  * @param {number[]} gain
  * @returns {number}
  */
function largestAltitude (gain) {
  let result = 0
  let current = result

  for (const altitude of gain) {
    current += altitude
    result = Math.max(result, current)
  }

  return result
}

function main () {
  const inputs = [
    [-5, 1, 5, 0, -7],
    [-4, -3, -2, -1, 4, 3, 2]
  ]

  for (const gain of inputs) {
    const result = largestAltitude(gain)
    console.log(result)
  }
}
main()
