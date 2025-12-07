import Data.List
import Data.Maybe

f (xs, n) s =
  let (xss, ns) = unzip [if s !! x == '^' then ([x - 1, x + 1], 1) else ([x], 0) | x <- xs]
  in (nub $ concat xss, n + sum ns)

merge = map (\xs -> (fst $ head xs, sum $ map snd xs)) . groupBy (\a b -> fst a == fst b)

f2 xs s = merge $ concat [if s !! x == '^' then [(x - 1, n), (x + 1, n)] else [(x, n)] | (x, n) <- xs]

parse s =
  let (l:ls) = lines s
  in (fromJust $ elemIndex 'S' l, ls)

sol1 s =
  let (x, ls) = parse s
  in snd $ foldl f ([x], 0) ls

sol2 s =
  let (x, ls) = parse s
  in sum $ map snd $ foldl f2 [(x, 1)] ls

input = readFile "7.txt"
test = fmap sol2 input
