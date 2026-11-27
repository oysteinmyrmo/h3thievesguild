<script lang="ts">
    import DisplayOptions from "./DisplayOptions.svelte";
    import StatChart from "./StatChart.svelte";
    import { chartFileName, chartPng, chartsZip, download, fileName, printable } from "../lib/export";
    import { game } from "../lib/game.svelte";
    import { router } from "../lib/router.svelte";
    import { categories, position } from "../lib/sections";
    import { theme } from "../lib/theme.svelte";
    import type { Dataset, StatInfo } from "../lib/wasm/pkg/h3wasm";

    /** One large chart: the one `stat` names, or the first. */
    let { dataset, stat }: { dataset: Dataset; stat?: string } = $props();

    const cats = $derived(categories(dataset.playerStats));
    const at = $derived(position(cats, stat));

    function show(to: StatInfo) {
        router.go({ page: "saves", stat: to.id });
    }

    /** The name of a neighbouring chart, with its category when it is in another one. */
    function neighbour(to: StatInfo): string {
        return at && to.group !== at.current.group ? `${to.name} (${to.group})` : to.name;
    }

    /** The left and right arrow keys step through the charts, unless typing or in a dialog. */
    function step(event: KeyboardEvent) {
        if (!at || event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) {
            return;
        }
        const target = event.target as HTMLElement | null;
        if (target?.closest("input, select, textarea, dialog")) {
            return;
        }
        const to = event.key === "ArrowLeft" ? at.previous : event.key === "ArrowRight" ? at.next : undefined;
        if (to) {
            event.preventDefault();
            show(to);
        }
    }

    /** What is being saved as an image: a statistic's ID, "all", or nothing. */
    let exporting = $state<string | undefined>(undefined);

    function exportSettings() {
        return { hidden: game.hidden, names: game.names, theme: theme.current, ...game.options };
    }

    async function savePng(info: StatInfo) {
        if (exporting) {
            return;
        }
        exporting = info.id;
        try {
            download(await chartPng(dataset, info, exportSettings()), chartFileName(dataset, info));
        } finally {
            exporting = undefined;
        }
    }

    async function saveAll() {
        if (exporting) {
            return;
        }
        exporting = "all";
        try {
            download(await chartsZip(dataset, exportSettings()), fileName(`${printable(dataset.map.name)} - charts`, "zip"));
        } finally {
            exporting = undefined;
        }
    }
</script>

<svelte:window onkeydown={step} />

{#if at}
    <div class="category-bar">
        <label>
            <span>Category</span>
            <select
                data-testid="category-select"
                value={at.category.group}
                onchange={(e) => {
                    const chosen = cats.find((c) => c.group === e.currentTarget.value);
                    if (chosen?.stats[0]) {
                        show(chosen.stats[0]);
                    }
                }}
            >
                {#each cats as category (category.group)}
                    <option value={category.group}>{category.group}</option>
                {/each}
            </select>
        </label>
    </div>

    <section class="view" aria-label={at.current.name}>
        <div class="main">
            <h2>{at.current.name}</h2>
            <div class="chart">
                <StatChart
                    {dataset}
                    stat={at.current}
                    hidden={game.hidden}
                    names={game.names}
                    compact={false}
                    options={game.options}
                />
            </div>
            <nav class="stepper" aria-label="Charts">
                <button
                    class="arrow previous"
                    data-testid="previous-chart"
                    title={`Previous chart: ${neighbour(at.previous)}`}
                    onclick={() => show(at.previous)}
                >
                    <span class="icon" aria-hidden="true">←</span>
                    <span class="text">
                        <span class="direction">Previous</span>
                        <span class="name">{neighbour(at.previous)}</span>
                    </span>
                </button>
                <select
                    data-testid="chart-select"
                    aria-label={`${at.category.group} chart`}
                    value={at.current.id}
                    onchange={(e) => {
                        const chosen = at.category.stats.find((s) => s.id === e.currentTarget.value);
                        if (chosen) {
                            show(chosen);
                        }
                    }}
                >
                    {#each at.category.stats as info (info.id)}
                        <option value={info.id}>{info.name}</option>
                    {/each}
                </select>
                <button
                    class="arrow next"
                    data-testid="next-chart"
                    title={`Next chart: ${neighbour(at.next)}`}
                    onclick={() => show(at.next)}
                >
                    <span class="text">
                        <span class="direction">Next</span>
                        <span class="name">{neighbour(at.next)}</span>
                    </span>
                    <span class="icon" aria-hidden="true">→</span>
                </button>
            </nav>
        </div>
        <aside class="side">
            <DisplayOptions />
            <div class="downloads">
                <button data-testid="download-chart" disabled={exporting !== undefined} onclick={() => savePng(at.current)}>
                    {exporting === at.current.id ? "Saving…" : "⤓ Download PNG"}
                </button>
                <button data-testid="download-all" disabled={exporting !== undefined} onclick={saveAll}>
                    {exporting === "all" ? "Saving charts…" : "⤓ Download all as PNG (zip)"}
                </button>
            </div>
        </aside>
    </section>
{/if}

<style>
    .category-bar {
        margin-bottom: 12px;
    }

    .category-bar label {
        display: inline-flex;
        align-items: center;
        gap: 10px;
        font-weight: 600;
    }

    select {
        font: inherit;
        padding: 6px 10px;
        border: 1px solid var(--border);
        border-radius: 8px;
        background: var(--surface);
        color: var(--text);
    }

    .category-bar select {
        font-size: 16px;
        font-weight: 600;
        min-width: 220px;
    }

    .view {
        display: grid;
        grid-template-columns: minmax(0, 1fr) 300px;
        gap: 16px;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        padding: 16px;
    }

    h2 {
        margin: 0;
        font-size: 18px;
    }

    .chart {
        height: min(68vh, 640px);
        margin-top: 8px;
    }

    .stepper {
        display: grid;
        /* Equal sides that may shrink, so the arrows stay the same size as each other. */
        grid-template-columns: minmax(0, 1fr) auto minmax(0, 1fr);
        align-items: center;
        gap: 12px;
        margin-top: 12px;
    }

    .stepper select {
        min-width: 240px;
        font-weight: 600;
    }

    /* The same size whatever the chart names, so nothing moves while stepping through. */
    .arrow {
        display: flex;
        align-items: center;
        gap: 10px;
        width: 300px;
        max-width: 100%;
        padding: 6px 12px;
    }

    .previous {
        justify-self: end;
        text-align: left;
    }

    .next {
        justify-self: start;
        text-align: right;
    }

    .icon {
        flex: none;
        font-size: 20px;
        line-height: 1;
    }

    .text {
        display: grid;
        flex: 1;
        min-width: 0;
    }

    .direction {
        color: var(--text-muted);
        font-size: 12px;
    }

    .name {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .side {
        display: grid;
        align-content: start;
        gap: 24px;
        border-left: 1px solid var(--border);
        padding-left: 16px;
    }

    .downloads {
        display: grid;
        gap: 8px;
    }
</style>
