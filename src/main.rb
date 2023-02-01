# @param x [Integer]
# @param y [Integer]
# @return [Integer]
def gcd(x, y)
  return x if y == 0

  gcd(y, x % y)
end

# @param str1 [String]
# @param str2 [String]
# @return [String]
def gcd_of_strings(str1, str2)
  return '' if (str1 + str2) != (str2 + str1)

  last = gcd(str1.size, str2.size) - 1
  str1[0..last]
end

def main
  inputs = [
    %w[ABCABC ABC],
    %w[ABABAB ABAB],
    %w[LEET CODE]
  ]

  inputs.each do |input|
    result = gcd_of_strings(input[0], input[1])
    puts(result)
  end
end

main
