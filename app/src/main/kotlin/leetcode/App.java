package leetcode;

class App {
  public static void main(String[] args) {
    Input[] inputs = {
        new Input(new int[] { 0, 1, 0, 3, 12 }),
        new Input(new int[] { 0 })
    };

    Solution solution = new Solution();
    for (Input input : inputs) {
      int[] nums = input.nums;
      solution.moveZeroes(nums);
      for (int i : nums) {
        System.out.print(i + ", ");
      }
      System.out.println();
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
  public void moveZeroes(int[] nums) {
    int lastZero = 0;

    for (int i = 0; i < nums.length; i += 1) {
      if (nums[i] != 0) {
        nums[lastZero] = nums[i];
        lastZero += 1;
      }
    }

    for (int i = lastZero; i < nums.length; i += 1) {
      nums[i] = 0;
    }
  }
}
