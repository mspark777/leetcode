defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @type accmap :: %{optional(integer) => {integer, pos_integer}}

  @spec average_of_levels(root :: TreeNode.t() | nil) :: [float]
  def average_of_levels(root) do
    %{}
    |> accumulate(root, 0)
    |> Enum.map(fn {level, {sum, count}} -> {level, sum / count} end)
    |> Enum.sort()
    |> Enum.map(&elem(&1, 1))
  end

  @spec accumulate(acc :: accmap, node :: nil | TreeNode.t(), level :: integer) :: accmap
  defp accumulate(acc, nil, _level), do: acc

  defp accumulate(acc, node, level) do
    Map.update(acc, level, {node.val, 1}, fn {sum, count} ->
      {sum + node.val, count + 1}
    end)
    |> accumulate(node.left, level + 1)
    |> accumulate(node.right, level + 1)
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        root:
          newnode(
            3,
            newval(9),
            newnode(20, newval(15), newval(7))
          )
      },
      %{
        root:
          newnode(
            3,
            newnode(9, newval(15), newval(7)),
            newval(20)
          )
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    root = input.root
    result = Solution.average_of_levels(root)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]), do: nil

  @spec newnode(val :: integer, left :: TreeNode.t() | nil, right :: TreeNode.t() | nil) ::
          TreeNode.t()
  defp newnode(val, left, right), do: %TreeNode{val: val, left: left, right: right}
  @spec newval(val :: integer) :: TreeNode.t()
  defp newval(val), do: newnode(val, nil, nil)
end

Main.main()
