defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
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



  def arr_to_tree(arr, i) do
    if i < tuple_size(arr) do
      val = elem(arr, i)
      if val == nil do
        nil
      else
        %TreeNode{
          val: elem(arr, i),
          left: arr_to_tree(arr, i * 2 + 1),
          right: arr_to_tree(arr, (i + 1) * 2)
        }
      end
    else
      nil
    end
  end

  def main() do
    inputs = [
      [0, 0, nil, 0, 0],
      [0, 0, nil, 0, nil, 0, nil, nil, 0]
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = min_camera_cover(input |> List.to_tuple |> arr_to_tree(0))
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Solution.main

