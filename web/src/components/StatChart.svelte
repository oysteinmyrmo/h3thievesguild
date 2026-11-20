<script lang="ts">
    import { onMount } from "svelte";
    import { playerShares, statChartOption, visiblePlayers, playerValues } from "../lib/chart";
    import { echarts } from "../lib/echarts";
    import { axisLabel, formatShare, formatValue } from "../lib/format";
    import { displayName, type PlayerNames } from "../lib/names";
    import type { ChartOptions } from "../lib/options";
    import { theme } from "../lib/theme.svelte";
    import type { Dataset, StatInfo } from "../lib/wasm/pkg/h3wasm";

    let {
        dataset,
        stat,
        hidden,
        names,
        compact,
        options,
    }: {
        dataset: Dataset;
        stat: StatInfo;
        hidden: ReadonlySet<string>;
        names: PlayerNames;
        compact: boolean;
        options: ChartOptions;
    } = $props();

    let element: HTMLDivElement;
    let chart = $state<ReturnType<typeof echarts.init> | undefined>(undefined);

    onMount(() => {
        chart = echarts.init(element);
        const resize = new ResizeObserver(() => chart?.resize());
        resize.observe(element);
        return () => {
            resize.disconnect();
            chart?.dispose();
        };
    });

    $effect(() => {
        const settings = { compact, hidden, names, theme: theme.current, ...options };
        chart?.setOption(statChartOption(dataset, stat.id, settings), { notMerge: true });
    });

    const players = $derived(visiblePlayers(dataset, hidden));
    const shares = $derived(options.chartType === "share" ? playerShares(dataset, players, stat.id) : undefined);

    /** A table cell: the share in the stacked view, otherwise the value. */
    function cell(playerId: string, values: (number | null)[], day: number): string {
        // A player out of the game has no share, rather than 0%.
        return shares && values[day] !== null ? formatShare(shares.get(playerId)?.[day]) : formatValue(values[day]);
    }
</script>

<div class="chart" bind:this={element} data-testid={`chart-${stat.id}`}></div>
<!-- The values as a table, for screen readers. A table cannot be shrunk itself, so its wrapper is. -->
<div class="sr-only">
<table data-testid={`table-${stat.id}`}>
    <caption>{stat.name} by day{shares ? ", as each player's share of the total" : ""}</caption>
    <thead>
        <tr>
            <th scope="col">Day</th>
            {#each players as player (player.id)}<th scope="col">{displayName(player, names)}</th>{/each}
        </tr>
    </thead>
    <tbody>
        {#each dataset.days as day, i (day.day)}
            <tr>
                <th scope="row">{axisLabel(day, options.xAxis)}</th>
                {#each players as player (player.id)}<td>{cell(player.id, playerValues(dataset, player, stat.id), i)}</td>{/each}
            </tr>
        {/each}
    </tbody>
</table>
</div>

<style>
    .chart {
        width: 100%;
        height: 100%;
    }
</style>
