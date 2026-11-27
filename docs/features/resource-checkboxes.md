# Choose resources with checkboxes (Economy)

**Why:** one chart per resource is many charts to step through; ticking the resources to see
is more flexible.

**Behaviour:**
- Gold keeps its own chart.
- The wood, mercury, ore, sulfur, crystal and gems charts become one "Resources" chart, with a
  checkbox per resource beside or above it, all ticked at first.
- The chart shows, per player, the sum of the ticked resources. With one box ticked it is that
  resource's chart as today.
- The ticked boxes are remembered with the display options.
- The tooltip lists the total and each ticked resource, such as
  "Red: 42 (wood 15, ore 20, gems 7)".
- With no box ticked, the chart says "Tick a resource to show it".
- Stacked share mode works on the sum.
- The PNG title names the ticked resources, such as "Resources: wood, ore".

**Data:** the existing per-resource series; the sum is computed in the frontend.

**Tests:** unit tests of the sum and the tooltip; browser test that ticks and unticks boxes,
checks the table behind the chart, and that the choice is remembered.

**Open:** whether a sum is right, or a line per resource per player (many lines), or one
player at a time.
