/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
  * @param {number} x
  * @param {bigint} n
  * @returns {number}
  */
function myPow1 (x, n) {
  if (n === 0n) {
    return 1
  }

  if (n < 0n) {
    n *= -1n
    x = 1 / x
  }

  let result = 1
  while (n !== 0n) {
    if ((n % 2n) === 1n) {
      result *= x
      n -= 1n
    }

    x *= x
    n /= 2n
  }

  return result
}

/**
  * @param {number} x
  * @param {number} n
  * @returns {number}
  */
function myPow (x, n) {
  return myPow1(x, BigInt(n))
}

function main () {
  const inputs = [
    { x: 2.00000, n: 10 },
    { x: 2.10000, n: 3 },
    { x: 2.00000, n: -2 }
  ]

  for (const { x, n } of inputs) {
    const result = myPow(x, n)
    console.log(result)
  }
}
main()
