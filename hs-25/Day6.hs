module Day6 where
import Data.List

op '+' = sum
op '*' = product

eval xs = op (head (last xs)) (map read (init xs))

sol1 s = sum (map eval (transpose (map words (lines s))))

spliti [] xs = [xs]
spliti (i:is) xs =
  let (a, b) = splitAt i xs
  in a:spliti (map (subtract i) is) b

nonempty = any (/=' ')

sol2 s =
  let ls = lines s
      (ops, offsets) = unzip [(x, i) | (x, i) <- zip (last ls) [0..], x /= ' ']
      rows = map (spliti (tail offsets)) (init ls)
      columns = map (map read . filter nonempty . transpose) (transpose rows)
  in sum (zipWith op ops columns)

input = readFile "6.txt"
test = fmap sol2 input
