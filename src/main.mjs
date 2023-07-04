/**
  * @param {number[]} nums
  * @returns {number}
  */
function singleNumber (nums) {
  let a = 0
  let b = 0

  for (const num of nums) {
    const tempa = (a & ~b & ~num) + (~a & b & num)
    const tempb = (~a & b & ~num) + (~a & ~b & num)

    a = tempa
    b = tempb
  }

  return a | b
}

function main () {
  const inputs = [
    [2, 2, 3, 2],
    [0, 1, 0, 1, 0, 1, 99]
  ]

  for (const input of inputs) {
    const result = singleNumber(input)
    console.log(result)
  }
}
main()
