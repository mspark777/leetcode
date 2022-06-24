defmodule Leetcode do
  def get_json(filename) do
    with {:ok, body} <- File.read(filename),
         {:ok, json} <- Poison.decode(body), do: {:ok, json}
  end

  def main() do
   # {:ok, data} = get_json('./src/data.json')
    inputs = [
      %{ target: [9, 3, 5] },
      %{ target: [1, 1, 1, 2] },
      %{ target: [8, 5] },
      %{ target: [1, 1000000000] },
     # %{ target: data["target"]}
    ]

    main(inputs)
  end

  def main([input | remains]) do
    result = Solution.is_possible(input.target)
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Leetcode.main()
