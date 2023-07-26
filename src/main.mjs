/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {string} num1
  * @param {string} num2
  * @returns {string}
  */
function multiply (num1, num2) {
  const result = BigInt(num1) * BigInt(num2)
  return result.toString()
}

function main () {
  const inputs = [
    ['2', '3'],
    ['123', '456']
  ]

  for (const [num1, num2] of inputs) {
    const result = multiply(num1, num2)
    console.log(result)
  }
}
main()
