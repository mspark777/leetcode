/**
 * @param {number[][]} img
 * @param {number[][]} kernel
 * @param {number} xshift
 * @param {number} yshift
 * @returns {number}
*/
function convolute (img, kernel, xshift, yshift) {
  const N = img.length

  let result = 0
  for (let r = 0; r < N; r += 1) {
    for (let c = 0; c < N; c += 1) {
      result += img[r][c] * kernel[r + yshift][c + xshift]
    }
  }

  return result
}

/**
 * @param {number[][]} img1
 * @param {number[][]} img2
 * @returns {number}
*/
function largestOverlap (img1, img2) {
  const N = img1.length
  const BN = (3 * N) - 2

  const bpadded = new Array(BN)
  for (let i = 0; i < BN; i += 1) {
    bpadded[i] = new Array(BN).fill(0)
  }

  for (let r = 0; r < N; r += 1) {
    for (let c = 0; c < N; c += 1) {
      bpadded[r + N - 1][c + N - 1] = img2[r][c]
    }
  }

  const SN = 2 * N - 1
  let maxOverlaps = 0
  for (let xshift = 0; xshift < SN; xshift += 1) {
    for (let yshift = 0; yshift < SN; yshift += 1) {
      maxOverlaps = Math.max(maxOverlaps, convolute(img1, bpadded, xshift, yshift))
    }
  }

  return maxOverlaps
}

async function main () {
  const inputs = [
    {
      img1: [[1, 1, 0], [0, 1, 0], [0, 1, 0]],
      img2: [[0, 0, 0], [0, 1, 1], [0, 0, 1]]
    },
    {
      img1: [[1]],
      img2: [[1]]
    },
    {

      img1: [[0]],
      img2: [[0]]
    }
  ]

  for (const { img1, img2 } of inputs) {
    const result = largestOverlap(img1, img2)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
