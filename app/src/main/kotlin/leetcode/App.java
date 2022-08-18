package leetcode;

class App {
  public static void main(String[] args) {
    Input[] inputs = {
        new Input(1),
        new Input(16),
        new Input(3)
    };

    Solution solution = new Solution();
    for (Input input : inputs) {
      int n = input.n;
      boolean result = solution.isPowerOfTwo(n);
      System.out.println(result);
    }
  }
}

class Input {
  public int n;

  Input(int n) {
    this.n = n;
  }

}

class Solution {
  public boolean isPowerOfTwo(int n) {
    return (n > 0) && ((n & (n - 1)) == 0);
  }
}
