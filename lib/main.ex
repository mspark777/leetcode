defmodule Solution do
  @spec add_digits(num :: integer) :: integer
  def add_digits(num) when num != 0, do: 1 + rem(num - 1, 9)
  def add_digits(_num), do: 0
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        n: 38
      },
      %{
        n: 0
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.n
    result = Solution.add_digits(n)
    IO.puts(result)
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
