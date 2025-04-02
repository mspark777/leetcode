import "@total-typescript/ts-reset";

function isBoomerang(points: number[][]): boolean {
  const [x0, y0] = points[0] as [number, number];
  const [x1, y1] = points[1] as [number, number];
  const [x2, y2] = points[2] as [number, number];

  return (y1 - y0) * (x2 - x1) !== (y2 - y1) * (x1 - x0);
}

interface Input {
  points: number[][];
}

function main(): void {
  const inputs: Input[] = [
    {
      points: [
        [1, 1],
        [2, 3],
        [3, 2],
      ],
    },
    {
      points: [
        [1, 1],
        [2, 2],
        [3, 3],
      ],
    },
  ];

  for (const input of inputs) {
    const result = isBoomerang(input.points);
    console.log(result);
  }
}
main();
