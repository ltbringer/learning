module Lib (
    grid,
    languages,
    formatGrid,
    outputGrid,
    findWord
) where

import Data.List (isInfixOf)
import Data.Maybe (catMaybes)

type Grid = [String]

grid = [ "__C________R___"
       , "__SI________U__"
       , "__HASKELL____B_"
       , "__A__A_____S__Y"
       , "__R___B___C____"
       , "__PHP____H_____"
       , "____S_LREP_____"
       , "____I__M_Y__L__"
       , "____L_E__T_O___"
       , "_________HB____"
       , "_________O_____"
       , "________CN_____"
       ]

languages = [ "BASIC"
            , "COBOL"
            , "CSHARP"
            , "HASKELL"
            , "LISP"
            , "PERL"
            , "PHP"
            , "PYTHON"
            , "RUBY"
            , "SCHEME"
            ]

-- | Display the grid.
outputGrid :: Grid -> IO ()
outputGrid grid = putStrLn $ formatGrid grid


-- | Format the grid.
formatGrid :: Grid -> String
formatGrid = unlines

findWordInLine :: String -> String -> Bool
findWordInLine = isInfixOf

-- | Find a word in the grid.
findWord :: Grid -> String -> Maybe String
findWord grid word =
    let lines = grid ++ (map reverse grid)
        match = or $ map (findWordInLine word) lines
    in if match then Just word else Nothing

findLanguage :: Grid -> [String] -> [String]
findLanguage grid languages = catMaybes $ map (\language -> (findWord grid language)) languages
