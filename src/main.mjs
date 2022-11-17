/**
 * @param {number} ax1
 * @param {number} ay1
 * @param {number} ax2
 * @param {number} ay2
 * @param {number} bx1
 * @param {number} by1
 * @param {number} bx2
 * @param {number} by2
 * @returns {number}
*/
function computeArea (ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) {
  const overX = Math.min(ax2, bx2) - Math.max(ax1, bx1)
  const overY = Math.min(ay2, by2) - Math.max(ay1, by1)

  const areaA = (ay2 - ay1) * (ax2 - ax1)
  const areaB = (by2 - by1) * (bx2 - bx1)
  const areaC = ((overX > 0) && (overY > 0)) ? overX * overY : 0

  return Math.abs(areaA) + Math.abs(areaB) - areaC
}

async function main () {
  const inputs = [
    [-3, 0, 3, 4, 0, -1, 9, 2],
    [-2, -2, 2, 2, -2, -2, 2, 2]
  ]

  for (const [ax1, ay1, ax2, ay2, bx1, by1, bx2, by2] of inputs) {
    const result = computeArea(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
