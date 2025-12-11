module Day10 where
import Data.List
import Data.List.Split
import qualified Data.Set as S
import Data.SBV
import Control.Monad
import Data.Maybe

parseb s = map f $ filter (not . null) $ splitOn " " s
  where f s = map read $ splitOn "," $ init $ tail s

parse :: String -> ([Bool], [[Int]], [Int])
parse s =
  let (ls, rest) = break (==']') $ tail s
      (bs, rest') = break (=='{') $ tail rest
      js = init $ tail rest'
  in (map (=='#') ls, parseb bs, map read $ splitOn "," js)

apply state b = [if i `elem` b then not x else x | (x, i) <- zip state [0..]]

apply2 state b = [if i `elem` b then x + 1 else x | (x, i) <- zip state [0..]]

search start target next = f [(start, 0)] (S.singleton start)
  where
    f ((state, n):q) visited
      | state == target = n
      | otherwise =
          let unvisited = filter (`S.notMember` visited) $ next state
              visited' = foldr S.insert visited unvisited
              q' = q ++ zip unvisited (repeat (n + 1))
          in f q' visited'

sol1 s =
  let xs = map parse $ lines s
      f (ls, bs, js) = search (replicate (length ls) False) ls next
        where next state = [apply state b | b <- bs]
  in sum $ map f xs

solvesbv (_, bs, js) = fromJust . getModelValue "total" <$> optLexicographic (
  do
    xs <- replicateM (length bs) sInteger_
    constrain $ sAll (.>= 0) xs
    constrain $ sAnd [fromIntegral j .== sum [x | (b, x) <- zip bs xs, i `elem` b] | (j, i) <- zip js [0..]]
    minimize "total" $ sum xs
  )

sol2 :: String -> IO Integer
sol2 s = 
  let xs = map parse $ lines s
  in sum <$> mapM solvesbv xs

input = readFile "10.txt"
-- test = fmap sol1 input
test = input >>= sol2
