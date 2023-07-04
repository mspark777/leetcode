import '@total-typescript/ts-reset'

function singleNumber (nums: number[]): number {
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

function main (): void {
  const inputs: number[][] = [
    [2, 2, 3, 2],
    [0, 1, 0, 1, 0, 1, 99]
  ]

  for (const input of inputs) {
    const result = singleNumber(input)
    console.log(result)
  }
}
main()
