defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Lib do
  @spec new_tree_node(val :: integer, left :: TreeNode.t(), right :: TreeNode.t()) :: TreeNode.t()
  def new_tree_node(val, left, right), do: %TreeNode{val: val, left: left, right: right}

  @spec new_tree_left(val :: integer, left :: TreeNode.t()) :: TreeNode.t()
  def new_tree_left(val, left), do: new_tree_node(val, left, nil)

  @spec new_tree_right(val :: integer, right :: TreeNode.t()) :: TreeNode.t()
  def new_tree_right(val, right), do: new_tree_node(val, nil, right)

  @spec new_tree_val(val :: integer) :: TreeNode.t()
  def new_tree_val(val), do: new_tree_node(val, nil, nil)
end
