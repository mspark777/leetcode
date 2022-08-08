defmodule Solution do
  @spec length_of_lis(nums :: [integer]) :: integer
  def length_of_lis(nums) do
    length_of_lis(nums, [])
  end

  def length_of_lis([num | nums], result) do
    reslen = length(result)
    index = binary_serach(result, num, 0, reslen - 1)

    cond do
      index == reslen -> length_of_lis(nums, result ++ [num])
      num < Enum.at(result, index) -> length_of_lis(
          nums,
          List.replace_at(result, index, num)
      )
      true -> length_of_lis(nums, result)
    end
  end

  def length_of_lis([], result), do: length(result)

  def binary_serach(nums, num, left, right) when left <= right do
    mid = div(left + right, 2)
    mvalue = Enum.at(nums, mid)
    if mvalue < num do
      binary_serach(nums, num, mid + 1, right)
    else
      binary_serach(nums, num, left, mid - 1)
    end
  end

  def binary_serach(_, _, left, _), do: left
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        nums: [10, 9, 2, 5, 3, 7, 101, 18]
      },
      %{
        nums: [0, 1, 0, 3, 2, 3]
      },
      %{
        nums: [7, 7, 7, 7, 7, 7, 7]
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums
    result = Solution.length_of_lis(nums)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
