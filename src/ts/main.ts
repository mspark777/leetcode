import "@total-typescript/ts-reset";

function canFormArray(arr: number[], pieces: number[][]): boolean {
  const pieceMap = new Map(pieces.map((piece) => [piece[0], piece]));
  let i = 0;

  while (i < arr.length) {
    const piece = pieceMap.get(arr[i]);
    if (piece == null) {
      return false;
    }

    for (const [j, num] of piece.entries()) {
      if (num !== arr[i + j]) {
        return false;
      }
    }

    i += piece.length;
  }

  return true;
}

interface Input {
  arr: number[];
  pieces: number[][];
}

function main(): void {
  const inputs: Input[] = [
    {
      arr: [15, 88],
      pieces: [[88], [15]],
    },
    {
      arr: [49, 18, 16],
      pieces: [[16, 18, 49]],
    },
    {
      arr: [91, 4, 64, 78],
      pieces: [[78], [4, 64], [91]],
    },
  ];

  for (const input of inputs) {
    const result = canFormArray(input.arr, input.pieces);
    console.log(result);
  }
}
main();
