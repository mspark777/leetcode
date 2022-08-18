defmodule Solution do
  use Bitwise

  @spec is_power_of_two(n :: integer) :: boolean
  def is_power_of_two(n) when n > 0, do: (n &&& n - 1) == 0
  def is_power_of_two(_), do: false
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        n: 1
      },
      %{
        n: 16
      },
      %{
        n: 3
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.n
    result = Solution.is_power_of_two(n)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
