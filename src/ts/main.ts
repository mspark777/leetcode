import { type TreeNode } from './lib'

function buildTree (_inorder: number[], _postorder: number[]): TreeNode | null {
  return null
}

async function main (): Promise<void> {
  const inputs: number[][][] = [
    [[9, 3, 15, 20, 7], [9, 15, 7, 20, 3]],
    [[-1], [-1]]
  ]

  for (const [inorder, postorder] of inputs) {
    const result = buildTree(inorder, postorder)
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
