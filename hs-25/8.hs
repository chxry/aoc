import Data.List
import Data.List.Split

pairs xs = [(x, y) | (x:ys) <- tails xs, y <- ys]

parseline s = let [a,b,c] = splitOn "," s
              in (read a, read b, read c)

parse s = map parseline $ lines s

distance (a, b, c) (d, e, f) = (d-a)^2 + (e-b)^2 + (f-c)^2

connect (i, j) xs =
    let (hit, rest) = partition (\x -> i `elem` x || j `elem` x) xs
    in concat hit:rest

shortestPairs xs = map snd $ sortOn fst $ map (\((a, i), (b, j)) -> (distance a b, (i, j))) $ pairs $ zip xs [0..]

clusters xs = map (:[]) [0.. length xs - 1]

sortDesc = sortBy (flip compare)

sol1 s =
  let xs = parse s
      ps = take 1000 $ shortestPairs xs
      cs = clusters xs
  in product $ take 3 $ sortDesc $ map length $ foldr connect cs ps

lastPair (p:ps) cs =
  let cs' = connect p cs
  in if length cs' == 1 then p else lastPair ps cs'

sol2 s = 
  let xs = parse s
      ps = shortestPairs xs
      cs = clusters xs
      (i, j) = lastPair ps cs
      getx i = let (x, _, _) = xs !! i in x
  in getx i * getx j

input = readFile "8.txt"
test = fmap sol2 input
