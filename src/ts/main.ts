import "@total-typescript/ts-reset";

class Solve {
  private tree: number[][];
  private distanceFromBob: number[];
  private readonly n: number;

  public constructor(edges: number[][], amount: number[]) {
    this.n = amount.length;
    this.tree = [];
    this.distanceFromBob = [];
    for (let i = 0; i < this.n; i += 1) {
      this.tree.push([]);
      this.distanceFromBob.push(0);
    }

    for (const edge of edges) {
      const [a, b] = edge as [number, number];
      this.tree[a]?.push(b);
      this.tree[b]?.push(a);
    }
  }

  public solve(bob: number, amount: number[]): number {
    return this.findPaths(0, 0, 0, bob, amount);
  }

  private findPaths(
    sourceNode: number,
    parentNode: number,
    time: number,
    bob: number,
    amount: number[],
  ): number {
    let maxIncome = 0;
    let maxChild = Number.MIN_SAFE_INTEGER;

    if (sourceNode === bob) {
      this.distanceFromBob[sourceNode] = 0;
    } else {
      this.distanceFromBob[sourceNode] = this.n;
    }

    for (const adjacentNode of this.tree[sourceNode] as number[]) {
      if (adjacentNode !== parentNode) {
        maxChild = Math.max(
          maxChild,
          this.findPaths(adjacentNode, sourceNode, time + 1, bob, amount),
        );
        this.distanceFromBob[sourceNode] = Math.min(
          this.distanceFromBob[sourceNode] as number,
          (this.distanceFromBob[adjacentNode] as number) + 1,
        );
      }
    }

    if ((this.distanceFromBob[sourceNode] as number) > time) {
      maxIncome += amount[sourceNode] as number;
    } else if (this.distanceFromBob[sourceNode] === time) {
      maxIncome += Number(BigInt(amount[sourceNode] as number) / 2n);
    }

    return maxChild === Number.MIN_SAFE_INTEGER
      ? maxIncome
      : maxIncome + maxChild;
  }
}

function mostProfitablePath(
  edges: number[][],
  bob: number,
  amount: number[],
): number {
  const solve = new Solve(edges, amount);
  return solve.solve(bob, amount);
}

interface Input {
  edges: number[][];
  bob: number;
  amount: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      edges: [
        [0, 1],
        [1, 2],
        [1, 3],
        [3, 4],
      ],
      bob: 3,
      amount: [-2, 4, 2, -4, 6],
    },
    {
      edges: [[0, 1]],
      bob: 1,
      amount: [-7280, 2350],
    },
  ];

  for (const input of inputs) {
    const result = mostProfitablePath(input.edges, input.bob, input.amount);
    console.log(result);
  }
}
main();
