import Data.List.Split
import Data.List

invalid1 :: String -> Bool
invalid1 xs = take (div (length xs) 2) xs == drop (div (length xs) 2) xs

invalidn :: String -> Int -> Bool
invalidn xs n | mod (length xs) n == 0 = alleq (chunksOf n xs)
              | otherwise = False

alleq (x:xs) = and (map (==x) xs)

invalid2 :: String -> Bool
invalid2 xs = or (map (invalidn xs) [1..length xs - 1])

range :: String -> [Int]
range x = let (a:b:[]) = splitOn "-" x
          in [read a..read b]

sol f s = sum (filter (\x -> f (show x)) (concat (map range (splitOn "," s))))

sol1 = sol invalid1
sol2 = sol invalid2

input = readFile "2.txt"
test = sol2 <$> input
