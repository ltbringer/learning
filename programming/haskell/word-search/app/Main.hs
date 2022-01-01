module Main where

import Lib (findWords, outputGrid)
import Data (grid, languages)

main :: IO ()
main = outputGrid $ findWords grid languages
