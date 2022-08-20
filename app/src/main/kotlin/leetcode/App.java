package leetcode;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.PriorityQueue;

class App {
  public static void main(String[] args) {
    Input[] inputs = {
        new Input(1, 1, new int[][] {}),
        new Input(100, 1, new int[][] { { 10, 100 } }),
        new Input(100, 10, new int[][] { { 10, 60 }, { 20, 30 }, { 30, 30 }, { 60, 40 } }),
        new Input(100, 50, new int[][] { { 50, 50 } }),
        new Input(100, 50, new int[][] { { 25, 50 }, { 50, 25 } })
    };

    Solution solution = new Solution();
    for (Input input : inputs) {
      int target = input.target;
      int startFuel = input.startFuel;
      int[][] stations = input.stations;
      int result = solution.minRefuelStops(target, startFuel, stations);
      System.out.println(result);
    }
  }
}

class Input {
  public int target;
  public int startFuel;
  public int[][] stations;

  Input(int target, int startFuel, int[][] stations) {
    this.target = target;
    this.startFuel = startFuel;
    this.stations = stations;
  }

}

class Solution {
  public int minRefuelStops(int target, int startFuel, int[][] stations) {
    final int NOT_FOUND = -1;
    PriorityQueue<Integer> queue = new PriorityQueue<>(Collections.reverseOrder());
    int tank = startFuel;
    int result = 0;
    ArrayList<int[]> list = new ArrayList<>(Arrays.asList(stations));
    list.add(new int[] { target, 0 });

    for (int[] station : list) {
      final int position = station[0];
      while (tank < position) {
        if (queue.isEmpty()) {
          return NOT_FOUND;
        }

        tank += queue.poll();
        result += 1;
      }
      final int fuel = station[1];
      queue.add(fuel);
    }

    return result;
  }

}
