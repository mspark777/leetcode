import "@total-typescript/ts-reset";

function squareIsWhite(coordinates: string): boolean {
  const first = coordinates.charCodeAt(0) - "a".charCodeAt(0);
  const second = Number(coordinates[1]);
  return (first + second) % 2 === 0;
}

interface Input {
  coordinates: string;
}

function main(): void {
  const inputs: Input[] = [
    {
      coordinates: "a1",
    },
    {
      coordinates: "h3",
    },
    {
      coordinates: "c7",
    },
  ];

  for (const input of inputs) {
    const result = squareIsWhite(input.coordinates);
    console.log(result);
  }
}
main();
