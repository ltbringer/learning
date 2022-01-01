module Lib (
    outputGrid,
    formatGrid,
    findWordInLine,
    getLines,
    skewRight,
    skewLeft,
    findWord,
    findWords,
) where

import Data.List (isInfixOf, transpose, intercalate)
import Data.Maybe (catMaybes)

type Grid = [String]

-- | Display the grid.
outputGrid :: Grid -> IO ()
outputGrid grid = putStrLn $ formatGrid grid


-- | Format the grid.
formatGrid :: Grid -> String
formatGrid = intercalate "\n"

findWordInLine :: String -> String -> Bool
findWordInLine = isInfixOf


getLines :: Grid -> [String]
getLines grid = 
    let horizontal = grid
        vertical = transpose horizontal
        diagonal1 = (transpose . skewLeft) grid
        diagonal2 = (transpose . skewRight) grid
        lines = horizontal ++ vertical ++ diagonal1 ++ diagonal2
    in lines ++ (map reverse lines)


skewRight :: Grid -> Grid
skewRight [] = []
skewRight (x:xs) = x : skewRight (map ((++) "_") xs)
    where indent line = '_' : line


skewLeft :: Grid -> Grid
skewLeft grid = skewRight (map reverse grid)


-- | Find a word in the grid.
findWord :: Grid -> String -> Maybe String
findWord grid word =
    let lines = getLines grid
        match = or $ map (findWordInLine word) lines
    in if match then Just word else Nothing


findWords :: Grid -> [String] -> [String]
findWords grid languages = catMaybes $ map (\language -> (findWord grid language)) languages
