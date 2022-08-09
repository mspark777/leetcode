defmodule Solution do
  @spec num_factored_binary_trees(arr :: [integer]) :: integer
  def num_factored_binary_trees(arr) do
    alen = length(arr)
    Enum.sort(arr)
    List.duplicate(1, alen)
  end

  defp create_index(m, [{v, i} | nums]) do
    create_index(Map.put(m, v, i), nums)
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        nums: [2, 4]
      },
      %{
        nums: [2, 4, 5, 10]
      },
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    nums = input.nums
    result = Solution.num_factored_binary_trees(nums)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
