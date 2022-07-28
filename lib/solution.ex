defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec postorder_traversal(root :: TreeNode.t | nil) :: [integer]
  def postorder_traversal(root) when root == nil, do: []

  def postorder_traversal(root) do
    postorder([root], [])
  end


  def postorder([top | stack], result) when top.left != nil and top.right != nil do
    val = top.val
    left = top.left
    right = top.right
    postorder([right, left] ++ stack, [val | result])
  end

  def postorder([top | stack], result) when top.left == nil and top.right != nil do
    val = top.val
    right = top.right
    postorder([right | stack], [val | result])
  end

  def postorder([top | stack], result) when top.left != nil and top.right == nil do
    val = top.val
    left = top.left
    postorder([left | stack], [val | result])
  end

  def postorder([top | stack], result)  do
    val = top.val
    postorder(stack, [val | result])
  end

  def postorder([], result), do: result
end
