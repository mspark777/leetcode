/**
 * @param {number[][]} moves
 * @return {string}
 */
var tictactoe = function (moves) {
  const row = [0, 0, 0];
  const col = [0, 0, 0];
  const diagonal = [0, 0];
  let point = 1;

  for (const [r, c] of moves) {
    row[r] += point;
    col[c] += point;
    if (r === c) {
      diagonal[0] += point;
    }

    if (r + c === 2) {
      diagonal[1] += point;
    }

    if (
      Math.abs(row[r]) === 3 ||
      Math.abs(col[c]) === 3 ||
      Math.abs(diagonal[0]) === 3 ||
      Math.abs(diagonal[1]) === 3
    ) {
      return point == 1 ? "A" : "B";
    }

    point = -point;
  }

  return moves.length === 9 ? "Draw" : "Pending";
};

/**
 * @typedef Input
 * @property {number[][]} moves
 */

/**
 * @return {undefined}
 */
function main() {
  /** @type Input[] */
  const inputs = [
    {
      moves: [
        [0, 0],
        [2, 0],
        [1, 1],
        [2, 1],
        [2, 2],
      ],
    },
    {
      moves: [
        [0, 0],
        [1, 1],
        [0, 1],
        [0, 2],
        [1, 0],
        [2, 0],
      ],
    },
    {
      moves: [
        [0, 0],
        [1, 1],
        [2, 0],
        [1, 0],
        [1, 2],
        [2, 1],
        [0, 1],
        [0, 2],
        [2, 2],
      ],
    },
  ];

  for (const input of inputs) {
    const result = tictactoe(input.moves);
    console.log(result);
  }
}
main();
