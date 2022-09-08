defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec inorder_traversal(root :: TreeNode.t() | nil) :: [integer]
  def inorder_traversal(nil), do: []

  def inorder_traversal(root),
    do: inorder_traversal(root.left) ++ [root.val] ++ inorder_traversal(root.right)
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        root: newright(1, newleft(2, newval(3)))
      },
      %{
        root: nil
      },
      %{
        root: newval(1)
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    root = input.root
    result = Solution.inorder_traversal(root)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]), do: nil

  defp newnode(val, left, right), do: %TreeNode{val: val, left: left, right: right}
  defp newval(val), do: newnode(val, nil, nil)
  defp newleft(val, left), do: newnode(val, left, nil)
  defp newright(val, right), do: newnode(val, nil, right)
end

Main.main()
