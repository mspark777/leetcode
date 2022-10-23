/**
 * @param {number[]} nums
 * @returns {number[]}
*/
function findErrorNums (nums) {
  const temps = new Array(nums.length).fill(0)
  for (const num of nums) {
    temps[num - 1] += 1
  }

  let missing = -1
  let dup = -1
  for (let i = 0; i < nums.length; i += 1) {
    const temp = temps[i]
    if (temp <= 0) {
      missing = i + 1
    } else if (temp > 1) {
      dup = i + 1
    }

    if ((missing >= 0) && (dup >= 0)) {
      break
    }
  }

  return [dup, missing]
}

async function main () {
  const inputs = [
    [1, 2, 2, 4],
    [1, 1]
  ]

  for (const nums of inputs) {
    const result = findErrorNums(nums)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
