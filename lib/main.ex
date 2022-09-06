defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec prune_tree(root :: TreeNode.t() | nil) :: TreeNode.t() | nil
  def prune_tree(nil), do: nil

  def prune_tree(root) do
    left = prune_tree(root.left)
    right = prune_tree(root.right)

    case {root.val, left, right} do
      {0, nil, nil} -> nil
      _ -> %TreeNode{val: root.val, left: left, right: right}
    end
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        root:
          newright(
            1,
            newnode(
              0,
              newval(0),
              newval(1)
            )
          )
      },
      %{
        root:
          newnode(
            1,
            newnode(
              0,
              newval(0),
              newval(0)
            ),
            newnode(
              1,
              newval(0),
              newval(1)
            )
          )
      },
      %{
        root:
          newnode(
            1,
            newnode(
              1,
              newleft(
                1,
                newval(0)
              ),
              newval(1)
            ),
            newnode(
              0,
              newval(0),
              newval(1)
            )
          )
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    root = input.root
    result = Solution.prune_tree(root)
    IO.puts(postorder([], result) |> Enum.join(", "))
    main(remains)
  end

  def main([]), do: nil

  defp newnode(val, left, right), do: %TreeNode{val: val, left: left, right: right}
  defp newval(val), do: newnode(val, nil, nil)
  defp newleft(val, left), do: newnode(val, left, nil)
  defp newright(val, right), do: newnode(val, nil, right)

  defp postorder(vals, nil), do: vals

  defp postorder(vals, node) do
    newvals =
      vals
      |> postorder(node.right)
      |> postorder(node.left)

    [node.val | newvals]
  end
end

Main.main()
