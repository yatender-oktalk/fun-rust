defmodule GCD do
  def calculate_gcd(m, 0) do
    m
  end

  def calculate_gcd(m, n) when m > n do
    calculate_gcd(n, rem(m, n))
  end

  def calculate_gcd(m, n) when n > m do
    calculate_gcd(m, rem(n, m))
  end
end
