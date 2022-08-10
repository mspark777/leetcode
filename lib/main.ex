defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec sorted_array_to_bst(nums :: [integer]) :: TreeNode.t | nil
  def sorted_array_to_bst(nums) do
    travel(nums, 0, length(nums))
  end

  defp travel(_, l, r) when l >= r, do: nil
  defp travel(nums, l, r) do
    mid = div(l + r, 2)
    %TreeNode{
      val: Enum.at(nums, mid),
      left: travel(nums, l, mid),
      right: travel(nums, mid + 1, r)
    }
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        nums: [-10, -3, 0, 5, 9]
      },
      %{
        nums: [1, 3]
      },
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums
    _result = Solution.sorted_array_to_bst(nums)
    #IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
