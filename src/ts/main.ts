import "@total-typescript/ts-reset";

function replaceElements(arr: number[]): number[] {
  const result = [...arr];
  const n = arr.length;
  let max = -1;

  for (const [i, num] of [...arr].reverse().entries()) {
    const j = n - (i + 1);
    result[j] = max;
    max = Math.max(max, num);
  }

  return result;
}

interface Input {
  arr: number[];
}

function main(): void {
  const inputs: Input[] = [
    {
      arr: [17, 18, 5, 4, 6, 1],
    },
    {
      arr: [400],
    },
  ];

  for (const input of inputs) {
    const result = replaceElements(input.arr);
    console.log(result);
  }
}
main();
