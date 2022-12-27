defmodule Leetcode do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([input | inputs]) do
    result =
      Solution.maximum_bags(
        input.capacity,
        input.rocks,
        input.additional_rocks
      )

    IO.puts(result)
    main(inputs)
  end

  def main([]) do
  end

  def main do
    main([
      %{
        capacity: [2, 3, 4, 5],
        rocks: [1, 2, 4, 4],
        additional_rocks: 2
      },
      %{
        capacity: [10, 2, 2],
        rocks: [2, 2, 0],
        additional_rocks: 100
      }
    ])
  end
end

Leetcode.main()
