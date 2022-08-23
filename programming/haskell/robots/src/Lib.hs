module Lib
    ( makeRobot,
    getName,
    getDmg,
    getHp,
    getStats,
    setName,
    setDmg,
    setHp,
    fight
    ) where

makeRobot (name, dmg, hp) = \message -> message (name, dmg, hp)

getName robot = robot (\(name, _, _) -> name)
getDmg robot = robot (\(_, dmg, _) -> dmg)
getHp robot = robot (\(_, _, hp) -> hp)
getStats robot = robot (\(name, dmg, hp) -> "Name: " ++ name ++ "\nDamage: " ++ show dmg ++ "\nHP: " ++ show hp)

setName robot newName = robot (\(name, dmg, hp) -> (newName, dmg, hp))
setDmg robot newDmg = robot (\(name, dmg, hp) -> (name, newDmg, hp))
setHp robot newHp = robot (\(name, dmg, hp) -> (name, dmg, newHp))

fight defender attacker = defender (\(n, d, h) -> makeRobot (n, d, max (h - (getDmg attacker)) 0))

combinations items = [(x, y) | 
    x <- items,
    y <- items,
    x /= y]

-- combat robots = map fight $ combinations robots
-- damage aRobot attackDamage = aRobot (\(n,a,h) ->
--                                       robot (n,a,h-attackDamage))