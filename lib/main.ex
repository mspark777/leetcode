defmodule Leetcode do
  def get_json(filename) do
    with {:ok, body} <- File.read(filename),
         {:ok, json} <- Poison.decode(body), do: {:ok, json}
  end

  def main() do
   # {:ok, data} = get_json('./src/data.json')
    inputs = [
      %{ nums: [4, 2, 3] },
      %{ nums: [4, 2, 1] },
      %{ nums: [1] },
      %{ nums: [1, 2] }
     # %{ target: data["target"]}
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.check_possibility(input.nums)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()
