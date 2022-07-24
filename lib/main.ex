defmodule Main do
  def main() do
    inputs = [
      %{
        nums: [1, 2, 3, 1]
      },
      %{
        nums: [1, 2, 3, 4]
      },
      %{
        nums: [1, 1, 1, 3, 3, 4, 3, 2, 4, 2]
      },
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.contains_duplicate(input.nums)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end
Main.main()
