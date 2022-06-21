import { furthestBuilding } from './ts/furthestBuilding'

async function main (): Promise<void> {
  const inputs = [
    { heights: [4, 2, 7, 6, 9, 14, 12], bricks: 5, leadders: 1 },
    { heights: [4, 12, 2, 7, 3, 18, 20, 3, 19], bricks: 10, leadders: 2 },
    { heights: [14, 3, 19, 3], bricks: 17, leadders: 0 },
    { heights: [1, 5, 1, 2, 3, 4, 10000], bricks: 4, leadders: 1 }
  ]

  for (const input of inputs) {
    const result = furthestBuilding(input.heights, input.bricks, input.leadders)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
