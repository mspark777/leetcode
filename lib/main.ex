defmodule Leetcode do
  def main() do
    inputs = [
      %{ n: "32" },
      %{ n: "82734" },
      %{ n: "27346209830709182346" },
      %{ n: "135" }
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.min_partitions(input.n)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()
