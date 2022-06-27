defmodule Leetcode do
  def main() do
    inputs = [
      %{
        target_sum: 22,
        root: %TreeNode{
          val: 5,
          left: %TreeNode{
            val: 4,
            left: %TreeNode{
              val: 11,
              left: %TreeNode{val: 7},
              right: %TreeNode{val: 2}
            }
          },
          right: %TreeNode{
            val: 8,
            left: %TreeNode{val: 13},
            right: %TreeNode{
              val: 4,
              left: nil,
              right: %TreeNode{val: 1}
            }
          }
        }
      },
      %{
        target_sum: 5,
        root: %TreeNode{val: 1,
          left: %TreeNode{val: 2},
          right: %TreeNode{val: 3}
        }
      },
      %{
        target_sum: 0,
        root: nil
      }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.has_path_sum(input.root, input.target_sum)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()
