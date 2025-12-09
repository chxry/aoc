module Util where
import Data.List

pairs xs = [(x, y) | (x:ys) <- tails xs, y <- ys]
