defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec is_same_tree(p :: TreeNode.t() | nil, q :: TreeNode.t() | nil) :: boolean
  def is_same_tree(nil, nil), do: true
  def is_same_tree(_, nil), do: false
  def is_same_tree(nil, _), do: false
  def is_same_tree(p, q) when p.val != q.val, do: false
  def is_same_tree(p, q), do: is_same_tree(p.left, q.left) && is_same_tree(p.right, q.right)
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([[p, q] | remains]) do
    result = Solution.is_same_tree(p, q)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      [
        newnode(1, newval(2), newval(3)),
        newnode(1, newval(2), newval(3))
      ],
      [
        newleft(1, newval(2)),
        newright(1, newval(2))
      ],
      [
        newnode(1, newval(2), newval(1)),
        newnode(1, newval(1), newval(2))
      ]
    ])
  end

  def newnode(val, left, right), do: %TreeNode{val: val, left: left, right: right}
  def newleft(val, left), do: newnode(val, left, nil)
  def newright(val, right), do: newnode(val, nil, right)
  def newval(val), do: newnode(val, nil, nil)
end

Main.main()
