defmodule Main do
  @spec main() :: nil
  def main() do
    inputs = [
      %{
        nums: [1, 3, 5]
      },
      %{
        nums: [1, 3, 5]
      }
    ]

    main(inputs)
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    NumArray.init_(input.nums)
    IO.puts(NumArray.sum_range(0, 2))
    NumArray.update(1, 2)
    IO.puts(NumArray.sum_range(0, 2))
    main(remains)
  end

  def main([]) do
    nil
  end
end

Main.main()
