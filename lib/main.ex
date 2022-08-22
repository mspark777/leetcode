defmodule Solution do
  use Bitwise

  @spec is_power_of_four(n :: integer) :: boolean
  def is_power_of_four(n) do
    n > 0 and (n &&& n - 1) == 0 and (n &&& 0x55555555) != 0
  end
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        n: 16
      },
      %{
        n: 5
      },
      %{
        n: 1
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.n
    result = Solution.is_power_of_four(n)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
