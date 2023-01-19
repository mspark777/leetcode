# @param i [Integer]
# @return [Integer]
def popcount(i)
  b = 0
  while i > 0
    i &= i - 1
    b += 1
  end
  b
end

# @param turned_on [Integer]
# @return [Array<String>]
def read_binary_watch(turned_on)
  result = []

  0.upto(11) do |h|
    0.upto(59) do |m|
      num = (h << 6) | m
      ones = popcount num
      if ones == turned_on
        if m < 10
          result.push("#{h}:0#{m}")
        else
          result.push("#{h}:#{m}")
        end
      end
    end
  end

  result
end

def main
  inputs = [1, 9]

  inputs.each do |turned_on|
    result = read_binary_watch turned_on
    puts result
  end
end

main
