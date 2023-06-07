/**
  * @param {number} n
  * @returns {number}
  */
function countSetBits (n) {
  let count = 0
  while (n > 0) {
    n &= (n - 1)
    count++
  }
  return count
}

/**
  * @param {number} a
  * @param {number} b
  * @param {number} c
  * @returns {number}
  */
function minFlips (a, b, c) {
  const d = (a | b) ^ c
  const e = a & b & d
  return countSetBits(d) + countSetBits(e)
}

function main () {
  const inputs = [
    [2, 6, 5],
    [4, 2, 7],
    [1, 2, 3],
    [7, 3, 9]
  ]

  for (const [a, b, c] of inputs) {
    const result = minFlips(a, b, c)
    console.log(result)
  }
}
main()
