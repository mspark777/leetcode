import { buildTree } from './solution'

interface Input {
  readonly preorder: number[]
  readonly inorder: number[]
}

async function main (): Promise<void> {
  const inputs: Input[] = [
    {
      preorder: [3, 9, 20, 15, 7],
      inorder: [9, 3, 15, 20, 7]
    },
    {
      preorder: [-1],
      inorder: [-1]
    }
  ]

  for (const input of inputs) {
    const result = buildTree(input.preorder, input.inorder)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
