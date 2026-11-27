# Best creature (Army)

**Why:** the strongest single creature a player has shows how far up the dwellings they are.

**Behaviour:** a "Best creature" chart in Army: per player and day, the highest AI value among
all creatures in their heroes' armies and town garrisons. The tooltip names the creature, such
as "Red: 4,872 (Black Dragon)".

**Data:** `Player::best_creature_ai_value` in the parser, plus the creature's name.

**Tests:** values against test saves with known armies (`creatures`, `bulwark`, `factory`);
a player with no creatures has 0.
