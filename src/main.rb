# @param s [String]
# @return [Integer]
def min_flips_mono_incr(s)
  result = 0
  num = 0

  s.each_char do |c|
    if c == '0'
      result = [num, result + 1].min
    else
      num += 1
    end
  end

  result
end

def main
  inputs = %w[
    00110
    010110
    00011000
  ]

  inputs.each do |s|
    result = min_flips_mono_incr s
    puts result
  end
end

main
