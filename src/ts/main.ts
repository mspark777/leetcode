import "@total-typescript/ts-reset";

function heightChecker(heights: number[]): number {
  const copy = heights.slice();
  copy.sort((a, b) => a - b);

  let result = 0;
  for (let i = 0; i < heights.length; i += 1) {
    if (heights[i] !== copy[i]) {
      result += 1;
    }
  }

  return result;
}

function main(): void {
  const inputs: number[][] = [
    [1, 1, 4, 2, 1, 3],
    [5, 1, 2, 3, 4],
    [1, 2, 3, 4, 5],
  ];

  for (const heights of inputs) {
    const result = heightChecker(heights);
    console.log(result);
  }
}
main();
