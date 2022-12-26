# @param {String} pattern
# @param {String} s
# @return {Boolean}
def word_pattern(pattern, s)
  patterns = pattern.split ''
  words = s.split ' '

  return false if patterns.length != words.length

  ptow = {}
  wtop = {}

  for i in 0..words.length
    word = words[i]
    p = patterns[i]

    if ptow.include?(p)
      return false if ptow[p] != word
    else
      ptow[p] = word
    end

    if wtop.include?(word)
      return false if wtop[word] != p
    else
      wtop[word] = p
    end
  end

  true
end

def main
  inputs = [
    ['abba', 'dog cat cat dog'],
    ['abba', 'dog cat cat fish'],
    ['aaaa', 'dog cat cat dog'],
    ['abba', 'dog dog dog dog']
  ]

  inputs.each do |input|
    result = word_pattern input[0], input[1]
    puts result
  end
end

main
