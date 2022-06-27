defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  def has_path_sum(nil, _) do
    false
  end

  def has_path_sum(root, target_sum) when root.left == nil and root.right == nil do
    root.val == target_sum
  end

  @spec has_path_sum(root :: TreeNode.t | nil, target_sum :: integer) :: boolean
  def has_path_sum(root, target_sum) do
    sum = target_sum - root.val
    has_path_sum(root.left, sum) or has_path_sum(root.right, sum)
  end

end
