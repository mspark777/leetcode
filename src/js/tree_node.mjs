/* eslint-disable @typescript-eslint/no-unsafe-argument */
/* eslint-disable @typescript-eslint/explicit-function-return-type */

export class TreeNode {
  /** @type {number} */
  val;
  /** @type {TreeNode|null} */
  left;
  /** @type {TreeNode|null} */
  right;

  /**
   * @param {number} val
   * @param {TreeNode|null} [left]
   * @param {TreeNode|null} [right]
   */
  constructor(val, left, right) {
    this.val = val ?? 0;
    this.left = left ?? null;
    this.right = right ?? null;
  }
}

/**
 * @param {Array<number|null>|null|undefined} items
 * @returns {TreeNode|null}
 */
export function toTree(items) {
  if (items == null) {
    return null;
  } else if (items.length < 1) {
    return null;
  }

  const iter = items.values();
  const first = iter.next();
  if (first.value == null) {
    return null;
  }

  const root = new TreeNode(first.value);
  const queue = [root];
  let i = 0;
  while (i < queue.length) {
    const node = queue[i];
    const left = iter.next();
    if (left.value != null) {
      node.left = new TreeNode(left.value);
      queue.push(node.left);
    }

    const right = iter.next();
    if (right.value != null) {
      node.right = new TreeNode(right.value);
      queue.push(node.right);
    }

    i += 1;
  }

  return root;
}
