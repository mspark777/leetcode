# @param n [Integer]
# @return [Boolean]
def can_win_nim(n)
  (n % 4) != 0
end

def main
  inputs = [
    4,
    1,
    2
  ]

  inputs.each do |input|
    result = can_win_nim input
    puts result
  end
end

main
