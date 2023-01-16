# @param s [String]
# @param t [String]
# @return [Character]
def find_the_difference(s, t)
  sum = 0
  s.each_codepoint do |p|
    sum ^= p
  end

  t.each_codepoint do |p|
    sum ^= p
  end

  sum.chr
end

def main
  inputs = [
    ['abcd', 'abcde'],
    ['', 'y']
  ]

  inputs.each do |input|
    result = find_the_difference input[0], input[1]
    puts result
  end
end

main
