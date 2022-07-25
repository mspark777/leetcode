defmodule Main do
  def main() do
    inputs = [
      %{
        nums: [5, 7, 7, 8, 8, 10], target: 8
      },
      %{
        nums: [5, 7, 7, 8, 8, 10], target: 6
      },
      %{
        nums: [], target: 0
      },
      %{
        nums: [1], target: 1
      }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.search_range(input.nums, input.target)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
