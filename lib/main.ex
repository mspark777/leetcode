defmodule Leetcode do
  def main() do
    inputs = [
        %{
            h: 5,
            w: 4,
            horizontal_cuts: [1, 2, 4],
            vertical_cuts: [1, 3],
        },
        %{
            h: 5,
            w: 4,
            horizontal_cuts: [3, 1],
            vertical_cuts: [1],
        },
        %{
            h: 5,
            w: 4,
            horizontal_cuts: [3],
            vertical_cuts: [3],
        },
        %{
            h: 1000000000,
            w: 1000000000,
            horizontal_cuts: [2],
            vertical_cuts: [2],
        },
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.max_area(input.h, input.w, input.horizontal_cuts, input.vertical_cuts)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()
