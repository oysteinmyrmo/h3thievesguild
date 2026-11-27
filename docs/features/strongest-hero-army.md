# Strongest hero army (Army)

**Why:** kingdom army strength is a total; this shows whether a player has one main army or
several small ones.

**Behaviour:** a "Strongest hero army" chart in Army: per player and day, the army strength of
their strongest hero, as in-game (with attack and defense).

**Data:** `Hero::army_strength` over the player's heroes on the map.

**Tests:** values against `kingdom_army_strength` test saves.

**Open:** whether heroes in town garrisons count; whether to also chart garrison strength
(towns only) so the two add up to kingdom army strength.
