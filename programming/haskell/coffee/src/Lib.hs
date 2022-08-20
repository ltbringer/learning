module Lib
    ( cup,
    getQty,
    drink,
    consume,
    isEmpty
    ) where


cup qty = (\f -> f qty)
getQty aCup = aCup (\qty -> qty)

drink aCup ml = cup $ max diff 0
    where
        contents = getQty aCup
        diff = contents - ml

consume aCup mls = foldl drink aCup mls

isEmpty aCup = (getQty aCup) == 0
