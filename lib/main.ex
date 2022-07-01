defmodule Leetcode do
  def main() do
    inputs = [
      %{ box_types: [[1, 3], [2, 2], [3, 1]], truck_size: 4 },
      %{ box_types: [[5, 10], [2, 5], [4, 7], [3, 9]], truck_size: 10 }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.maximum_units(input.box_types, input.truck_size)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()
