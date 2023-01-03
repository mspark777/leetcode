defmodule Solution do
  @spec min_deletion_size(strs :: [String.t()]) :: integer
  def min_deletion_size(strs) do
    strs
    |> Enum.map(&String.to_charlist/1)
    |> List.zip()
    |> Enum.map(&Tuple.to_list/1)
    |> Enum.map(&check/1)
    |> Enum.count(& &1)
  end

  def check([]), do: false
  def check([_ | []]), do: false

  @spec check(chars :: charlist()) :: boolean
  def check([first | remains]) do
    second = hd(remains)

    if first <= second do
      check(remains)
    else
      true
    end
  end
end

defmodule Main do
  @moduledoc """
  Documentation for `Leetcode`.
  """

  @spec main(inputs :: [[String.t()]]) :: nil
  def main([strs | remains]) do
    result = Solution.min_deletion_size(strs)

    IO.puts(result)
    main(remains)
  end

  def main([]) do
  end

  def main do
    main([
      ["cba", "daf", "ghi"],
      ["a", "b"],
      ["zyx", "wvu", "tsr"]
    ])
  end
end

Main.main()
