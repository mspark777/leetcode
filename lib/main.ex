defmodule Main do
  import Solution

  def main() do
    inputs = [
      [4,2,4,5,6]
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = maximum_unique_subarray(input)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
