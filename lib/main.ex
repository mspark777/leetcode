defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        root: %TreeNode{
          val: 1,
          right: %TreeNode{
            val: 2,
            left: %TreeNode{
              val: 3
            }
          }
        }
      },
      %{
        root: nil
      },
      %{
        root: %TreeNode{
          val: 1
        }
      },
      %{
        root: %TreeNode{
          val: 1,
          left: %TreeNode{
            val: 2,
          }
        }
      },
      %{
        root: %TreeNode{
          val: 3,
          left: %TreeNode{
            val: 1
          },
          right: %TreeNode{
            val: 2,
          }
        }
      },
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    result = Solution.postorder_traversal(input.root)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
