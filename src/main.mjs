// import { createRequire } from 'module'
import { createTreeFromArray, rightSideView } from './solution.mjs'

async function main () {
  const inputs = [
    {
      root: [1, 2, 3, null, 5, null, 4]
    },
    {
      root: [1, null, 3]
    },
    {
      root: []
    }
  ]

  for (const input of inputs) {
    const result = rightSideView(createTreeFromArray(input.root))
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
