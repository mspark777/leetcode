defmodule Solution do
  @spec majority_element(nums :: [integer]) :: integer
  def majority_element(nums), do: majority_element(nums, 0, 0)

  @spec majority_element(nums :: [integer], count :: integer, candidate :: integer) :: integer
  defp majority_element([num | nums], count, candidate) do
    new_candidate = if count < 1, do: num, else: candidate
    new_count = if num == new_candidate, do: count + 1, else: count - 1

    majority_element(nums, new_count, new_candidate)
  end

  defp majority_element([], _, candidate), do: candidate
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        nums: [3, 2, 3]
      },
      %{
        nums: [2, 2, 1, 1, 1, 2, 2]
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums
    result = Solution.majority_element(nums)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
