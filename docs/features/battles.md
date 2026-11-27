# Battles

**Why:** big battles decide a game. As in the charts at the end of an Age of Empires II game,
marking them where they happened shows why the lines go as they do, and who fought whom.

## Behaviour

- Each big battle is marked on the charts by a vertical line on its day, like the elimination
  lines. Hovering it tells what happened, such as "Day 51: Green beat Red. Red lost 114,000,
  Green lost 42,000." (creature value, as below)
- A list of the battles of the game, each with the creatures each player lost, such as "Red
  lost 3 Angels, 3 Archangels, 13 Champions…, from day 51 to day 52".
- Detection need not be perfect. Where it guesses, it says so ("probably Green").

## Detecting a battle

Compare each save with the save of the day before.

- **Creatures lost:** per player, each creature type that has fewer in the player's heroes and
  garrisons than the day before. An upgrade is not a loss: a drop of one type matched by the same
  rise of its upgrade counts as neither.
- **Creature value lost:** the AI value of those creatures. Use this, not kingdom army strength.
  Kingdom army strength counts the heroes' attack and defense, so it drops when the main hero hands
  the army to a weak hero, or comes back from a town garrison. In `long_game`, Green's kingdom army
  strength drops 15% from day 45 to 46 while the creature value rises 10%. Orange's drops 19% from
  day 53 to 54, while Orange lost only a Hobgoblin and a Nomad.
- **Loss per type, not net value:** recruiting and weekly growth on the same day can hide a loss
  in the totals, so losses are counted per type.
- **Big:** a loss above a threshold, both as a share of the player's own creature value and of
  the largest army in the game that day. To be tuned on `long_game`.
- **Heroes lost:** a hero the player had the day before and not now was defeated, or retreated
  or surrendered.
- **The other side:** a hero of another player who gained experience, and lost creatures or
  defeated a hero, most likely fought it. The closer that hero was to the loser's last position,
  the more likely. Experience gained in a battle is the hit points of the creatures killed, plus
  500 for a hero defeated. With no such hero, the battle was against neutral creatures, or not
  clear.
- Several battles can happen on one day, and one player can lose to several others.

## What `long_game` shows

These are the battles the frontend should find. "Day N" is the change from the save of day N to
the save of day N + 1.

| Day | What the saves show | Battle |
|---|---|---|
| 11 | Blue's Derek, Tan's Ayden and Green's Rosic are gone with their armies; Red's Cuthbert loses half his army and gains a level. No other player's hero gains experience to match. | Four separate battles, most likely against neutral creatures. |
| 22 | Tan's Ayden, at (15, 39), is gone with all of Tan's army. Red's Cuthbert, at (14, 42) the day before, gains 921 experience and loses 4 Monks and 12 Pikemen. | Red beat Tan. |
| 39 | Purple's Uland and Elleshar, at (37, 42), are gone and Purple is out. Orange's Arlach, at (43, 45), gains 3,365 experience and loses 27 Harpy Hags, 9 Thunderbirds and a Cyclops. | Orange beat Purple. |
| 51 | Red's Cuthbert, at (59, 52), is gone with 3 Angels, 3 Archangels, 13 Champions, 26 Crusaders, 77 Marksmen, 18 Royal Griffins and 10 Zealots. Green's Rosic, at (53, 60), gains 4,020 experience and loses 27 Lizard Warriors, 18 Mighty Gorgons, 12 Wyvern Monarchs and 2 Dragon Flies. | Green beat Red. |
| 53 | Red's Cuthbert, rehired, is gone with his army. Green's Rosic loses his whole army without moving and without gaining experience, and stays on the map. | Not clear; perhaps both armies died, such as by Armageddon. |
| 58 | Red's Cuthbert and Nagash are gone, and Red is out. Orange's Arlach gains 3,030 experience, ends next to where Nagash was, and loses Efreet Sultans, Cyclopes, Devils and more. | Orange beat Red. |

## Data

- Each player's creatures by type per day, from [Army composition](army-composition.md), which
  is built first.
- The data set gets, per player and day, the creature value: AI value without the heroes' attack
  and defense. Creature value could also be a chart of its own.
- The data set builder finds the battles and adds them to the data set: day, the players and
  heroes on each side, how sure the guess is, and the creatures and creature value each side lost.
- The hero positions and experience it needs are already read.

## Tests

- Unit tests of the detection: a loss, an upgrade, a handover between heroes, a hero going into a
  garrison, recruiting on the day of a loss.
- The battles in `long_game` in the table above, and no battle on days with only handovers.
- Browser test of the battle lines and their tooltips.

## Open

- The thresholds for a big battle.
- Whether battles against neutral creatures are shown, or only those between players.
- Whether the lines are on every chart or only the Army charts, and where the battle list goes.
- How to treat a day without a save: a loss across it is a battle on one of the days in between.
- Things that change armies other than battles: Necromancy raising Skeletons, Resurrection,
  creatures joining, dismissed stacks, armies given away.
- Whether a retreated or surrendered hero can be told from a defeated one, from the hero's
  record afterwards.
- Town captures (a town's owner changes) as events of their own.
- Day 53 in `long_game` above.
