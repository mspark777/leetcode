import "@total-typescript/ts-reset";

function generateTheString(n: number): string {
  return (n & 1) === 1 ? "a".repeat(n) : "a".repeat(n - 1) + "b";
}

interface Input {
  n: number;
}

function main(): void {
  const inputs: Input[] = [
    {
      n: 4,
    },
    {
      n: 2,
    },
    {
      n: 7,
    },
  ];

  for (const input of inputs) {
    const result = generateTheString(input.n);
    console.log(result);
  }
}
main();
