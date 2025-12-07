import Data.List
import Data.Maybe

f xs s = nub $ concat [if s !! x == '.' then [x] else [x - 1, x + 1] | x <- xs]

merge = map (\xs -> (fst (head xs), sum (map snd xs))) . groupBy (\a b -> fst a == fst b)

f2 xs s = merge $ concat [if s !! x == '.' then [(x, n)] else [(x - 1, n), (x + 1, n)] | (x, n) <- xs]

parse s =
  let (l:ls) = lines s
  in (fromJust $ elemIndex 'S' l, ls)

sol1 s =
  let (x, ls) = parse s
  in sum $ zipWith (\l xs -> length $ filter (\x -> l !! x /= '.') xs) ls (scanl f [x] ls)

sol2 s =
  let (x, ls) = parse s
  in sum $ map snd $ foldl f2 [(x, 1)] ls

input = readFile "7.txt"
test = fmap sol2 input
