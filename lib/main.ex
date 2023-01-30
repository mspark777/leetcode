defmodule Solution do
  @spec tribonacci(n :: integer) :: integer
  def tribonacci(n) when n < 3, do: if(n < 1, do: 0, else: 1)

  def tribonacci(n), do: recursive(n, 0, 1, 1)

  def recursive(n, t0, t1, t2) when n > 2 do
    recursive(n - 1, t1, t2, t0 + t1 + t2)
  end

  def recursive(_, _, _, t2), do: t2
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  def main([n | remains]) do
    result = Solution.tribonacci(n)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      4,
      25
    ])
  end
end

Main.main()
