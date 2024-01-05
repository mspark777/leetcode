defmodule Hello do
  def hello do
    :world
  end
end

defmodule Main do
  def main do
    IO.puts(Hello.hello())
  end
end

Main.main()
