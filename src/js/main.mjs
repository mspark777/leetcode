/**
  * @param {number[]} flowerbed
  * @param {number} n
  * @returns {boolean}
  */
function canPlaceFlowers (flowerbed, n) {
  let count = 0
  const last = flowerbed.length - 1
  for (let i = 0; i <= last; i += 1) {
    const plot = flowerbed[i]
    if (plot > 0) {
      continue
    }

    const emptyLeft = (i === 0) || (flowerbed[i - 1] === 0)
    const emptyRight = (i === last) || (flowerbed[i + 1] === 0)
    if (emptyLeft && emptyRight) {
      flowerbed[i] = 1
      count += 1
      if (count >= n) {
        return true
      }
    }
  }

  return count >= n
}

async function main () {
  const inputs = [
    { flowerbed: [1, 0, 0, 0, 1], n: 1 },
    { flowerbed: [1, 0, 0, 0, 1], n: 2 }
  ]

  for (const { flowerbed, n } of inputs) {
    const result = canPlaceFlowers(flowerbed, n)
    console.log(result)
  }
}

main()
  .then(() => {
    process.exit(0)
  }).catch(e => {
    console.error(e)
    process.exit(1)
  })
