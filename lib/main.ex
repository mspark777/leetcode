defmodule Leetcode do
  def main() do
    inputs = [
      %{ people: [[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]] },
      %{ people: [[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]] }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.reconstruct_queue(input.people)
    result
    |> Enum.map(fn person -> Enum.join(person, ", ") end)
    |> Enum.join(" | ")
    |> IO.puts
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()
