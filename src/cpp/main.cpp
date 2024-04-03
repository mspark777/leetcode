#include <iostream>
#include <string>
#include <vector>

class Solution {
 public:
  bool exist(std::vector<std::vector<char>>& board, std::string word) {
    const int r = board.size();
    const int c = board[0].size();

    for (int i = 0; i < r; i += 1) {
      for (int j = 0; j < c; j += 1) {
        if (board[i][j] == word[0] && this->dfs(i, j, 0, board, word)) {
          return true;
        }
      }
    }

    return false;
  }

 protected:
  bool dfs(int r, int c, int count, std::vector<std::vector<char>>& board,
           std::string& word) {
    if (static_cast<int>(word.length()) == count) {
      return true;
    } else if (r < 0) {
      return false;
    } else if (r >= static_cast<int>(board.size())) {
      return false;
    } else if (c < 0) {
      return false;
    } else if (c >= static_cast<int>(board[0].size())) {
      return false;
    } else if (board[r][c] != word[count]) {
      return false;
    }

    const char temp = board[r][c];
    board[r][c] = ' ';

    const bool result = dfs(r - 1, c, count + 1, board, word) ||
                        dfs(r + 1, c, count + 1, board, word) ||
                        dfs(r, c - 1, count + 1, board, word) ||
                        dfs(r, c + 1, count + 1, board, word);

    board[r][c] = temp;

    return result;
  }
};

struct Input {
  std::vector<std::vector<char>> board;
  std::string word;
};

int main() {
  const Input inputs[] = {
      {{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
       "ABCCED"},
      {{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
       "SEE"},
      {{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
       "ABCB"},
  };

  for (auto input : inputs) {
    Solution s;
    std::cout << s.exist(input.board, input.word) << std::endl;
  }
  return 0;
}
