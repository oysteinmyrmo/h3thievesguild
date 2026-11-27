# Ask to reload a remembered game that lacks newer charts

**Why:** the page remembers the last game as processed data, not as save files. After a new
chart is added, a remembered game has no data for it, and the chart is silently missing.

**Behaviour:** when the remembered game lacks any of the statistics the current version has,
show a notice above the charts: "This game was loaded with an older version. Choose the save
files again to see all charts." The old charts still show until then.

**Data:** compare the statistic IDs of the remembered data set with those the parser has now.

**Tests:** unit test of the comparison; browser test that stores an older data set and checks
the notice, and that it goes away after loading the saves.
