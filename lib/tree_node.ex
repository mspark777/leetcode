defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil

  def new_node(val, left, right), do: %TreeNode{val: val, left: left, right: right}

  def new_left(val, left), do: new_node(val, left, nil)
  def new_right(val, right), do: new_node(val, nil, right)
  def new_val(val), do: new_node(val, nil, nil)
end
