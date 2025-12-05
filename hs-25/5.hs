import Data.List.Split
import Data.List
import Data.Function

range :: String -> (Int, Int)
range s =
  let [a,b] = splitOn "-" s
  in (read a, read b)

parse :: String -> ([(Int, Int)], [Int])
parse s =
  let [fresh, avail] = splitOn [""] (lines s)
  in (map range fresh, map read avail)

sol1 s =
  let (fresh, avail) = parse s
  in length (filter (\x -> any (\(a,b) -> x >= a && x <= b) fresh) avail)

sol2 s =
  let (fresh, avail) = parse s
      f [] r = [r]
      f (r@(a, b):rs) r'@(a', b')
        | a' <= b = (a, max b b'):rs
        | otherwise = r':r:rs
      ranges = foldl f [] (sortOn fst fresh)
  in sum (map (\(a, b) -> b - a + 1) ranges)

input = readFile "5.txt"
test = fmap sol2 input
