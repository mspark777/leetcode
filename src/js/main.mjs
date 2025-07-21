/* eslint-disable @typescript-eslint/explicit-function-return-type */

/**
 * @param {number[]} distance
 * @param {number} start
 * @param {number} destination
 * @return {number}
 */
var distanceBetweenBusStops = function (distance, start, destination) {
  if (start > destination) {
    const t = start;
    start = destination;
    destination = t;
  }

  const clockwise = distance
    .slice(start, destination)
    .reduce((acc, i) => acc + i);
  const total = distance.reduce((acc, i) => acc + i);
  return Math.min(clockwise, total - clockwise);
};

function main() {
  const inputs = [
    [[1, 2, 3, 4], 0, 1],
    [[1, 2, 3, 4], 0, 2],
    [[1, 2, 3, 4], 0, 3],
  ];

  for (const [distance, start, destination] of inputs) {
    const result = distanceBetweenBusStops(distance, start, destination);
    console.log(result);
  }
}
main();
