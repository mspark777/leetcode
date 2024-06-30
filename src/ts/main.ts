import "@total-typescript/ts-reset";

class UnionFind {
  private readonly parents: number[];
  private components: number;
  public constructor(n: number) {
    this.parents = [];
    this.components = n;
    for (let i = 0; i <= n; i += 1) {
      this.parents.push(i);
    }
  }

  public find(x: number): number {
    if (x !== this.parents[x]) {
      this.parents[x] = this.find(this.parents[x] as number);
    }

    return this.parents[x] as number;
  }

  public union(x: number, y: number): boolean {
    x = this.find(x);
    y = this.find(y);

    if (x === y) {
      return false;
    }

    this.parents[y] = x;
    this.components -= 1;
    return true;
  }

  public isConnected(): boolean {
    return this.components === 1;
  }
}

function maxNumEdgesToRemove(n: number, edges: number[][]): number {
  const alice = new UnionFind(n);
  const bob = new UnionFind(n);

  let edgesNeed = 0;
  for (const edge of edges) {
    const [t, u, v] = edge as [number, number, number];
    if (t !== 3) {
      continue;
    }

    const aliceOK = alice.union(u, v);
    const bobOK = bob.union(u, v);
    if (aliceOK || bobOK) {
      edgesNeed += 1;
    }
  }

  for (const edge of edges) {
    const [t, u, v] = edge as [number, number, number];
    if (t === 1) {
      if (alice.union(u, v)) {
        edgesNeed += 1;
      }
    } else if (t === 2) {
      if (bob.union(u, v)) {
        edgesNeed += 1;
      }
    }
  }

  if (alice.isConnected() && bob.isConnected()) {
    return edges.length - edgesNeed;
  }

  return -1;
}

function main(): void {
  const inputs: Array<[number[][], number]> = [
    [
      [
        [3, 1, 2],
        [3, 2, 3],
        [1, 1, 3],
        [1, 2, 4],
        [1, 1, 2],
        [2, 3, 4],
      ],
      4,
    ],
    [
      [
        [3, 1, 2],
        [3, 2, 3],
        [1, 1, 4],
        [2, 1, 4],
      ],
      4,
    ],
    [
      [
        [3, 2, 3],
        [1, 1, 2],
        [2, 3, 4],
      ],
      4,
    ],
  ];

  for (const [edges, n] of inputs) {
    const result = maxNumEdgesToRemove(n, edges);
    console.log(result);
  }
}
main();
