import "@total-typescript/ts-reset";

function reorderSpaces(text: string): string {
  const splitted = text.split(" ");
  const words = splitted.filter((word) => word !== "");
  const spacePerWord = Math.floor((splitted.length - 1) / (words.length - 1));
  const joined = words.join(
    " ".repeat(Number.isFinite(spacePerWord) ? spacePerWord : 0),
  );
  return joined + " ".repeat(text.length - joined.length);
}

interface Input {
  text: string;
}

function main(): void {
  const inputs: Input[] = [
    {
      text: "  this   is  a sentence ",
    },
    {
      text: " practice   makes   perfect",
    },
  ];

  for (const input of inputs) {
    const result = reorderSpaces(input.text);
    console.log(result);
  }
}
main();
