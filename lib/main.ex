defmodule Main do
  def main() do
    inputs = [
      %{
        ratings: [1, 0, 2]
      },
      %{
        ratings: [1, 2, 2]
      },
      %{
        ratings: [1, 2, 87, 87, 87, 2, 1]
      },
      %{
        ratings: [1, 2, 3, 5, 4, 3, 2, 1, 4, 3, 2, 1, 3, 2, 1, 1, 2, 3, 4]
      },
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.partition(input.ratings)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end
Main.main()
