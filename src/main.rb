# @param s [String]
# @param t [String]
# @return [Boolean]
def is_subsequence(s, t)
  return true if s.length < 1

  si = 0

  t.each_char do |c|
    si += 1 if s[si] == c
    return true if s.length == si
  end

  false
end

def main
  inputs = [
    %w[abc ahbgdc], %w[axc ahbgdc],
    ['', ''], ['', 'ahbgdc']
  ]

  inputs.each do |input|
    result = is_subsequence input[0], input[1]
    puts result
  end
end

main
