defmodule StackNode do
  @type t :: %__MODULE__{
          len: integer,
          num: integer,
          digit: integer
        }
  defstruct len: nil, num: nil, digit: nil
end

defmodule Solution do
  @spec nums_same_consec_diff(n :: integer, k :: integer) :: [integer]
  def nums_same_consec_diff(n, k) do
    1..9
    |> Enum.map(&newnode(n - 1, &1, &1))
    |> dfs(k, [])
  end

  @spec dfs(stack :: [StackNode.t()], k :: integer, result :: [integer]) :: [integer]
  defp dfs([%StackNode{len: len, num: num, digit: _} | nodes], k, result) when len == 0,
    do: dfs(nodes, k, [num | result])

  defp dfs([%StackNode{len: len, num: num, digit: digit} | nodes], k, result) do
    newnodes =
      0..9
      |> Enum.map(&{&1, abs(&1 - digit)})
      |> Enum.filter(&(elem(&1, 1) == k))
      |> Enum.map(&elem(&1, 0))
      |> Enum.map(&newnode(len - 1, num * 10 + &1, &1))

    dfs(newnodes ++ nodes, k, result)
  end

  defp dfs([], _, result), do: result

  @spec newnode(len :: integer, num :: integer, digit :: integer) :: StackNode.t()
  defp newnode(len, num, digit), do: %StackNode{len: len, num: num, digit: digit}
end

defmodule Main do
  @spec main() :: nil
  def main() do
    main([
      %{
        n: 3,
        k: 7
      },
      %{
        n: 2,
        k: 1
      }
    ])
  end

  @spec main(list[any]) :: nil
  def main([input | remains]) do
    n = input.n
    k = input.k
    result = Solution.nums_same_consec_diff(n, k)
    IO.puts(result |> Enum.join(", "))
    main(remains)
  end

  def main([]), do: nil
end

Main.main()
