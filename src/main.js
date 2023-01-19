/**
 * @param {number[]} nums
 * @param {number} k
 * @returns {number}
 */
function subarraysDivByK (nums, k) {
  let prefixMod = 0
  let result = 0

  const modGroups = new Array(k).fill(0)
  modGroups[0] = 1

  for (const num of nums) {
    prefixMod = (prefixMod + (num % k) + k) % k
    result += modGroups[prefixMod]
    modGroups[prefixMod] += 1
  }

  return result
}

async function main () {
  const inputs = [
    { nums: [4, 5, 0, -2, -3, 1], k: 5 },
    { nums: [5], k: 9 }
  ]

  for (const { nums, k } of inputs) {
    const result = subarraysDivByK(nums, k)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
