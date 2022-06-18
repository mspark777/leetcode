defmodule WordFilter do
  @spec init_(words :: [String.t]) :: any
  def init_(words) do
    pid = Process.whereis(__MODULE__)
    if pid == nil do
      Agent.start_link(fn -> words end, name: __MODULE__)
    else
      Agent.update(pid, fn _ -> words end)
    end
    pid = Process.whereis(__MODULE__)

    parent = self()
    wordslen = Enum.count(words)
    Enum.with_index(words, fn word, i ->
      spawn(fn ->
        last = String.length(word) - 1
        Enum.each(0..last, fn j ->
          prefix = String.slice(word, 0..j)
          Enum.each(0..last, fn k ->
            suffix = String.slice(word, k..last)
            key = Enum.join([prefix, suffix], "#")
            Agent.update(pid, fn substirngs ->
              index = Map.get(substirngs, key, -1)
              if index < i do
                Map.put(substirngs, key, i)
              else
                substirngs
              end
            end)
          end)
        end)
        send(parent, :end)
      end)
    end)

    wait(0, wordslen)
  end

  def wait(current, total) do
    if current < total do
      receive do
        :end -> wait(current + 1, total)
      end
    end
  end

  @spec f(prefix :: String.t, suffix :: String.t) :: integer
  def f(prefix, suffix) do
    pid = Process.whereis(__MODULE__)
    words = Agent.get(pid, fn words -> words end)
    total = Enum.count(words)
  end
end

defmodule Solution do
  def main() do
    inputs = [
      %{words: ["apple"], ps: ["a", "e"]},
      %{words: ["apple"], ps: ["a", "e"]}
    ]

    main(inputs)
  end

  def main([input | remains]) do
    WordFilter.init_(input.words)
    result = WordFilter.f(Enum.at(input.ps, 0), Enum.at(input.ps, 1))
    IO.puts(result)
    main(remains)
  end

  def main([]) do
    nil
  end
end

Solution.main

