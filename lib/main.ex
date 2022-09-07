defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec tree2str(root :: TreeNode.t() | nil) :: String.t()
  def tree2str(nil), do: ""

  def tree2str(%TreeNode{val: val, left: left, right: right}) when left == nil and right == nil,
    do: to_string(val)

  def tree2str(%TreeNode{val: val, left: left, right: right}) when right == nil,
    do: "#{val}(#{tree2str(left)})"

  def tree2str(%TreeNode{val: val, left: left, right: right}),
    do: "#{val}(#{tree2str(left)})(#{tree2str(right)})"
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        root:
          newnode(
            1,
            newleft(2, newval(4)),
            newval(3)
          )
      },
      %{
        root:
          newnode(
            1,
            newright(2, newval(4)),
            newval(3)
          )
      },
      %{
        root:
          newleft(
            -1,
            newleft(-2, newleft(-3, newval(-4)))
          )
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    root = input.root
    result = Solution.tree2str(root)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil

  defp newnode(val, left, right), do: %TreeNode{val: val, left: left, right: right}
  defp newval(val), do: newnode(val, nil, nil)
  defp newleft(val, left), do: newnode(val, left, nil)
  defp newright(val, right), do: newnode(val, nil, right)
end

Main.main()
