# @param word [String]
# @return [Boolean]
def detect_capital_use(word)
  return true if word == word.upcase
  return true if word == word.downcase

  word == word[0].upcase + word[1..-1].downcase
end

def main
  inputs = %w[
    USA
    Google
    leetcode
    FlaG
  ]

  inputs.each do |word|
    result = detect_capital_use word
    puts result
  end
end

main
