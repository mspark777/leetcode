defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec max_depth(root :: TreeNode.t() | nil) :: integer
  def max_depth(root), do: travel(root, 0)

  def travel(nil, depth), do: depth

  def travel(node, depth) do
    d = depth + 1
    max(travel(node.left, d), travel(node.right, d))
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([root | remains]) do
    result = Solution.max_depth(root)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      newnode(3, newval(9), newnode(20, newval(15), newval(7))),
      newright(1, newval(2))
    ])
  end

  def newnode(val, left, right) do
    %TreeNode{val: val, left: left, right: right}
  end

  def newright(val, right) do
    %TreeNode{val: val, left: nil, right: right}
  end

  def newval(val) do
    %TreeNode{val: val, left: nil, right: nil}
  end
end

Main.main()
