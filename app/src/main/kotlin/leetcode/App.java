package leetcode;

import java.util.HashMap;

class App {
  public static void main(String[] args) {
    Input[] inputs = {
        new Input(new int[] { 1, 2, 3, 3, 4, 5 }),
        new Input(new int[] { 1, 2, 3, 3, 4, 4, 5, 5 }),
        new Input(new int[] { 1, 2, 3, 4, 4, 5 })
    };

    Solution solution = new Solution();
    for (Input input : inputs) {
      int[] nums = input.nums;
      boolean result = solution.isPossible(nums);
      System.out.println(result);
    }
  }
}

class Input {
  public int[] nums;

  Input(int[] nums) {
    this.nums = nums;
  }

}

class Solution {
  public boolean isPossible(int[] nums) {
    HashMap<Integer, Integer> lefts = new HashMap<>();
    HashMap<Integer, Integer> ends = new HashMap<>();

    for (int num : nums) {
      lefts.put(num, lefts.getOrDefault(num, 0) + 1);
    }

    for (int cur : nums) {
      int left = lefts.getOrDefault(cur, 0);
      if (left == 0) {
        continue;
      }

      lefts.put(cur, left - 1);

      final int before1 = cur - 1;
      final int ebefore1 = ends.getOrDefault(before1, 0);
      if (ebefore1 > 0) {
        ends.put(before1, ebefore1 - 1);
        ends.put(cur, ends.getOrDefault(cur, 0) + 1);
        continue;
      }

      final int after1 = cur + 1;
      final int after2 = cur + 2;
      final int lafter1 = lefts.getOrDefault(after1, 0);
      final int lafter2 = lefts.getOrDefault(after2, 0);
      if ((lafter1 > 0) && (lafter2 > 0)) {
        lefts.put(after1, lafter1 - 1);
        lefts.put(after2, lafter2 - 1);
        ends.put(after2, ends.getOrDefault(after2, 0) + 1);
        continue;
      }

      return false;
    }

    return true;
  }
}
