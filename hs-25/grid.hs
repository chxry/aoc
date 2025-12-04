module Grid where
import qualified Data.Vector as V

data Grid a = G Int Int (V.Vector a)
  deriving (Eq)

width :: Grid a -> Int
width (G w h v) = w

height :: Grid a -> Int
height (G w h v) = h

getList :: Grid a -> [a]
getList (G w h v) = V.toList v

getVector :: Grid a -> V.Vector a
getVector (G w h v) = v

gridFromList :: Int -> Int -> [a] -> Grid a
gridFromList w h xs = G w h (V.fromList xs)

gridFromLists :: [[a]] -> Grid a
gridFromLists xs = G (length (head xs)) (length xs) (V.fromList (concat xs))

readGrid :: String -> Grid Char
readGrid = gridFromLists . lines

get :: Grid a -> (Int, Int) -> a
get (G w h v) (x, y) = v V.! (x + y * w)

inGrid :: Grid a -> (Int, Int) -> Bool
inGrid (G w h v) (x, y) = x >= 0 && x < w && y >= 0 && y < h

getDefault :: Grid a -> a -> (Int, Int) -> a
getDefault g@(G w h v) d c@(x, y) | inGrid g c = V.unsafeIndex v (x + y * w)
                                    | otherwise = d

neighbours :: Grid a -> a -> (Int, Int) -> [a]
neighbours g d (x, y) = [getDefault g d (x + dx, y + dy) | dx <- [-1, 0, 1], dy <- [-1, 0, 1], dx /= 0 || dy /= 0]

mapGrid :: (a -> b) -> Grid a -> Grid b
mapGrid f (G w h v) = G w h (V.map f v)
