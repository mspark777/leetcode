defmodule Main do
  def main() do
    inputs = [
      [4,2,4,5,6]
    ]

    main(inputs)
  end

  def main([input | remains]) do
    import MaximumUniqueSubarray
    result = maximum_unique_subarray(input)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
