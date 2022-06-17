defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec min_depth(root :: TreeNode.t | nil) :: integer
  def min_depth(root) do
    cond do
      root == nil -> 0
      root.left  != nil and root.right != nil -> min(min_depth(root.left), min_depth(root.right)) + 1
      true -> max(min_depth(root.left), min_depth(root.right)) + 1
    end
  end

  def main() do
    inputs = [
      %TreeNode{
        val: 3,
        left: %TreeNode {
          val: 9, left: nil, right: nil
        },
        right: %TreeNode {
          val: 20,
          left: %TreeNode {
            val: 15, left: nil, right: nil
          },
          right: %TreeNode {
            val: 7, left: nil, right: nil
          }
        }
      },

      %TreeNode {
        val: 2,
        left: nil,
        right: %TreeNode {
          val: 3,
          left: nil,
          right: %TreeNode {
            val: 4,
            left: nil,
            right: %TreeNode {
              val: 5,
              left: nil,
              right: %TreeNode {
                val: 6,
                left: nil,
                right: nil
              }
            }
          }
        }
      }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = min_depth(input)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Solution.main

