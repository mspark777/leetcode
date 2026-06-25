import "@total-typescript/ts-reset";

class _Node {
  val: boolean;
  isLeaf: boolean;
  topLeft: _Node | null;
  topRight: _Node | null;
  bottomLeft: _Node | null;
  bottomRight: _Node | null;
  constructor(
    val?: boolean,
    isLeaf?: boolean,
    topLeft?: _Node,
    topRight?: _Node,
    bottomLeft?: _Node,
    bottomRight?: _Node,
  ) {
    this.val = val === undefined ? false : val;
    this.isLeaf = isLeaf === undefined ? false : isLeaf;
    this.topLeft = topLeft === undefined ? null : topLeft;
    this.topRight = topRight === undefined ? null : topRight;
    this.bottomLeft = bottomLeft === undefined ? null : bottomLeft;
    this.bottomRight = bottomRight === undefined ? null : bottomRight;
  }
}

function intersect(
  quadTree1: _Node | null,
  quadTree2: _Node | null,
): _Node | null {
  if (quadTree1 == null) {
    return quadTree2;
  }

  if (quadTree2 == null) {
    return quadTree1;
  }

  if (quadTree1.isLeaf) {
    if (quadTree1.val) {
      return new _Node(true, true);
    } else {
      return quadTree2;
    }
  }

  if (quadTree2.isLeaf) {
    if (quadTree2.val) {
      return new _Node(true, true);
    } else {
      return quadTree1;
    }
  }

  const topLeft = intersect(quadTree1.topLeft, quadTree2.topLeft);
  const topRight = intersect(quadTree1.topRight, quadTree2.topRight);
  const bottomLeft = intersect(quadTree1.bottomLeft, quadTree2.bottomLeft);
  const bottomRight = intersect(quadTree1.bottomRight, quadTree2.bottomRight);

  if (
    topLeft?.isLeaf === true &&
    topRight?.isLeaf === true &&
    bottomLeft?.isLeaf === true &&
    bottomRight?.isLeaf === true &&
    topLeft.val == topRight.val &&
    topRight.val == bottomLeft.val &&
    bottomLeft.val == bottomRight.val
  ) {
    return new _Node(topLeft.val, true);
  }

  return new _Node(
    false,
    false,
    topLeft ?? undefined,
    topRight ?? undefined,
    bottomLeft ?? undefined,
    bottomRight ?? undefined,
  );
}

interface Input {
  nums: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      nums: [1, 3, 6, 10, 12, 15],
    },
    {
      nums: [1, 2, 4, 7, 10],
    },
    {
      nums: [4, 4, 9, 10],
    },
  ];

  for (const input of inputs) {
    const result = averageValue(input.nums);
    console.log(result);
  }
}
main();
