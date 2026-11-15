<script lang="ts">
    import { onMount } from "svelte";
    import { statChartOption, visiblePlayers, playerValues } from "../lib/chart";
    import { echarts } from "../lib/echarts";
    import { formatValue } from "../lib/format";
    import { readTheme } from "../lib/theme";
    import type { Dataset, StatInfo } from "../lib/types";

    let {
        dataset,
        stat,
        hidden,
        compact,
    }: { dataset: Dataset; stat: StatInfo; hidden: ReadonlySet<string>; compact: boolean } = $props();

    let element: HTMLDivElement;
    let chart = $state<ReturnType<typeof echarts.init> | undefined>(undefined);
    let theme = $state(readTheme());

    onMount(() => {
        chart = echarts.init(element);
        const resize = new ResizeObserver(() => chart?.resize());
        resize.observe(element);
        const scheme = matchMedia("(prefers-color-scheme: dark)");
        const retheme = () => (theme = readTheme());
        scheme.addEventListener("change", retheme);
        return () => {
            scheme.removeEventListener("change", retheme);
            resize.disconnect();
            chart?.dispose();
        };
    });

    $effect(() => {
        chart?.setOption(statChartOption(dataset, stat.id, { compact, hidden, theme }), { notMerge: true });
    });

    const players = $derived(visiblePlayers(dataset, hidden));
</script>

<div class="chart" bind:this={element} data-testid={`chart-${stat.id}`}></div>
<!-- The values as a table, for screen readers. A table cannot be shrunk itself, so its wrapper is. -->
<div class="sr-only">
<table data-testid={`table-${stat.id}`}>
    <caption>{stat.name} by day</caption>
    <thead>
        <tr>
            <th scope="col">Day</th>
            {#each players as player (player.id)}<th scope="col">{player.name}</th>{/each}
        </tr>
    </thead>
    <tbody>
        {#each dataset.days as day, i (day.day)}
            <tr>
                <th scope="row">{day.label}</th>
                {#each players as player (player.id)}<td>{formatValue(playerValues(dataset, player, stat.id)[i])}</td>{/each}
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
