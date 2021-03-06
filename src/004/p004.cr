struct Int
  def palindrome? : Bool
    self == to_s.reverse.to_i
  end
end

def solve
  largest = 0
  (100..999).each do |x|
    (100..999).each do |y|
      product = x * y
      largest = product if product > largest && product.palindrome?
    end
  end
  puts largest
end

puts solve
