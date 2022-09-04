import { newTreeLeft, newTreeNode, newTreeVal } from './lib.mjs'

/**
 * @param {import('./lib.mjs').TreeNode} root
 * @return {number[][]}
 */
function verticalTraversal (root) {
  const verticals = new Map()
  const stack = [{
    row: 0,
    col: 0,
    node: root
  }]

  for (let top = stack.pop(); top != null; top = stack.pop()) {
    const { row, col, node } = top
    if (node == null) {
      continue
    }

    const vertical = verticals.get(col) ?? []
    vertical.push({
      row,
      col,
      value: node.val
    })

    verticals.set(col, vertical)
    stack.push({
      row: row + 1,
      col: col - 1,
      node: node.left
    }, {
      row: row + 1,
      col: col + 1,
      node: node.right
    })
  }

  const result = []
  for (const nodes of verticals.values()) {
    nodes.sort((a, b) => a.row === b.row ? a.value - b.value : a.row - b.row)
    result.push({
      col: nodes[0].col,
      values: nodes.map(n => n.value)
    })
  }

  return result
    .sort((a, b) => a.col - b.col)
    .map(r => r.values)
}

async function main () {
  const inputs = [
    {
      root: newTreeNode(
        3,
        newTreeVal(9),
        newTreeNode(20,
          newTreeVal(15),
          newTreeVal(7)
        )
      )
    },
    {
      root: newTreeNode(
        1,
        newTreeNode(
          2,
          newTreeVal(4),
          newTreeVal(5)
        ),
        newTreeNode(
          3,
          newTreeVal(6),
          newTreeVal(7)
        )
      )
    },
    {
      root: newTreeNode(
        1,
        newTreeNode(
          2,
          newTreeVal(4),
          newTreeVal(6)
        ),
        newTreeNode(
          3,
          newTreeVal(5),
          newTreeVal(7)
        )
      )
    },
    {
      root: newTreeNode(
        3,
        newTreeNode(
          1,
          newTreeVal(0),
          newTreeVal(2)
        ),
        newTreeLeft(
          4,
          newTreeVal(2)
        )
      )
    }
  ]

  for (const { root } of inputs) {
    const result = verticalTraversal(root)
    console.log(result)
  }
}

main().catch(e => {
  console.error(e)
  process.exit(1)
})
