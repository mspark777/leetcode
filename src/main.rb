# @param s {Array<String>}
# @return {Void} Do not return anything, modify s in-place instead.
def reverse_string(s)
  i = 0
  j = s.length - 1
  while i < j
    s[i], s[j] = s[j], s[i]
    i += 1
    j -= 1
  end
end

def main
  inputs = [
    %w[h e l l o],
    %w[H a n n a h]
  ]

  inputs.each do |s|
    reverse_string s
    puts s.join ', '
  end
end

main
