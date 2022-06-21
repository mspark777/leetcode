defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule MinCameraCover do
  @spec min_camera_cover(root :: TreeNode.t | nil) :: integer
  def min_camera_cover(root) do
    case travel(root, 0) do
      {:leaf, depth} -> depth + 1
      {_, depth} -> depth
    end
  end

  def travel(nil, depth) do
    {:nocamera, depth}
  end

  def travel(node, depth) do
    {left, ldepth} = travel(node.left, depth)
    {right, rdepth}  = travel(node.right, ldepth)
    cond do
      left == :leaf or right == :leaf -> {:camera, rdepth + 1}
      left == :camera or right == :camera -> {:nocamera, rdepth}
      true -> {:leaf, rdepth}
    end
  end
end
