<script lang="ts">
    import ErrorPanel from "./ErrorPanel.svelte";
    import GameHeader from "./GameHeader.svelte";
    import MissingDaysDialog from "./MissingDaysDialog.svelte";
    import StatChart from "./StatChart.svelte";
    import { game } from "../lib/game.svelte";
    import { loadSaves, readFiles } from "../lib/load";
    import DisplayOptions from "./DisplayOptions.svelte";
    import { chartFileName, chartPng, chartsZip, download, fileName, printable } from "../lib/export";
    import { theme } from "../lib/theme.svelte";
    import type { StatInfo } from "../lib/wasm/pkg/h3wasm";
    import { router } from "../lib/router.svelte";
    import type { Dataset } from "../lib/wasm/pkg/h3wasm";

    let { stat }: { stat?: string } = $props();

    /** A loaded data set with missing days, waiting for the user to confirm. */
    let pending = $state<Dataset | undefined>(undefined);
    let input: HTMLInputElement;

    async function choose(event: Event) {
        const files = (event.currentTarget as HTMLInputElement).files;
        if (!files || files.length === 0) {
            return;
        }
        game.loading = true;
        game.error = undefined;
        const result = await loadSaves(await readFiles(files));
        game.loading = false;
        input.value = "";
        if (!result.ok) {
            game.error = result.error;
            scrollTo(0, 0);
        } else if (result.dataset.missingDaysWarning) {
            pending = result.dataset;
        } else {
            game.show(result.dataset);
            router.go({ page: "saves" });
        }
    }

    function confirmMissing(proceed: boolean) {
        if (proceed && pending) {
            game.show(pending);
            router.go({ page: "saves" });
        }
        pending = undefined;
    }

    const stats = $derived(game.dataset?.playerStats ?? []);
    const detailIndex = $derived(stats.findIndex((s) => s.id === stat));
    const detail = $derived(stats[detailIndex]);
    // The charts before and after this one, wrapping around at the ends.
    const previous = $derived(stats[(detailIndex - 1 + stats.length) % stats.length]);
    const next = $derived(stats[(detailIndex + 1) % stats.length]);

    /** What is being saved as an image: a statistic's ID, "all", or nothing. */
    let exporting = $state<string | undefined>(undefined);

    function exportSettings() {
        return { hidden: game.hidden, names: game.names, theme: theme.current, ...game.options };
    }

    async function savePng(info: StatInfo) {
        if (!game.dataset || exporting) {
            return;
        }
        exporting = info.id;
        try {
            download(await chartPng(game.dataset, info, exportSettings()), chartFileName(game.dataset, info));
        } finally {
            exporting = undefined;
        }
    }

    async function saveAll() {
        if (!game.dataset || exporting) {
            return;
        }
        exporting = "all";
        try {
            const name = fileName(`${printable(game.dataset.map.name)} - charts`, "zip");
            download(await chartsZip(game.dataset, exportSettings()), name);
        } finally {
            exporting = undefined;
        }
    }

    /** The left and right arrow keys step through the charts, unless typing or in a dialog. */
    function step(event: KeyboardEvent) {
        if (!detail || event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) {
            return;
        }
        const target = event.target as HTMLElement | null;
        if (target?.closest("input, select, textarea, dialog")) {
            return;
        }
        const to = event.key === "ArrowLeft" ? previous : event.key === "ArrowRight" ? next : undefined;
        if (to) {
            event.preventDefault();
            router.go({ page: "saves", stat: to.id });
        }
    }
</script>

<svelte:window onkeydown={step} />

<div class="toolbar">
    <h1>Save games</h1>
    <label class="button primary">
        Choose save files…
        <input
            bind:this={input}
            class="sr-only"
            type="file"
            multiple
            data-testid="file-input"
            onchange={choose}
            disabled={game.loading}
        />
    </label>
    {#if game.dataset}
        <button onclick={() => game.clear()}>Clear</button>
    {/if}
</div>

{#if game.loading}
    <p class="status" role="status">Reading the save files…</p>
{/if}

{#if game.error}
    <ErrorPanel error={game.error} />
{/if}

{#if pending?.missingDaysWarning}
    <MissingDaysDialog message={pending.missingDaysWarning} onanswer={confirmMissing} />
{/if}

{#if game.dataset}
    {#if game.restoredAt}
        <p class="status" data-testid="restored">Showing the game you loaded last time.</p>
    {/if}
    <GameHeader dataset={game.dataset} />
    {#if detail}
        <div class="detail-toolbar">
            <a class="button all-charts" href="#/saves">← All charts</a>
            {#if previous && next}
                <nav class="stepper" aria-label="Other charts">
                    <a class="button" href={`#/saves/${previous.id}`} title={previous.name} data-testid="previous-chart">
                        ← Previous chart
                    </a>
                    <a class="button" href={`#/saves/${next.id}`} title={next.name} data-testid="next-chart">
                        Next chart →
                    </a>
                </nav>
            {/if}
            <button
                class="save-png"
                data-testid="download-chart"
                disabled={exporting !== undefined}
                onclick={() => savePng(detail)}
            >
                {exporting === detail.id ? "Saving…" : "⤓ Download PNG"}
            </button>
        </div>
        <section class="detail" aria-label={detail.name}>
            <h2>{detail.name}</h2>
            <div class="detail-body">
                <div class="detail-chart">
                    <StatChart
                        dataset={game.dataset}
                        stat={detail}
                        hidden={game.hidden}
                        names={game.names}
                        compact={false}
                        options={game.options}
                    />
                </div>
                <aside class="options-column">
                    <DisplayOptions />
                </aside>
            </div>
        </section>
    {:else}
        <div class="overview">
            <section class="grid" aria-label="All charts">
                {#each game.dataset.playerStats as info (info.id)}
                    <!-- The button sits beside the link, not in it: a link cannot hold a button. -->
                    <div class="card-wrap">
                        <a class="card" href={`#/saves/${info.id}`} data-testid={`card-${info.id}`}>
                            <h3>{info.name}</h3>
                            <span class="group">{info.group}</span>
                            <div class="card-chart">
                                <StatChart
                                    dataset={game.dataset}
                                    stat={info}
                                    hidden={game.hidden}
                                    names={game.names}
                                    compact={true}
                                    options={game.options}
                                />
                            </div>
                        </a>
                        <button
                            class="card-png"
                            data-testid={`download-${info.id}`}
                            title={`Download ${info.name} as PNG`}
                            aria-label={`Download ${info.name} as PNG`}
                            disabled={exporting !== undefined}
                            onclick={() => savePng(info)}
                        >
                            {exporting === info.id ? "…" : "⤓"}
                        </button>
                    </div>
                {/each}
            </section>
            <aside class="overview-options">
                <DisplayOptions />
                <button data-testid="download-all" disabled={exporting !== undefined} onclick={saveAll}>
                    {exporting === "all" ? "Saving charts…" : "⤓ Download all as PNG (zip)"}
                </button>
            </aside>
        </div>
    {/if}
{:else if !game.loading && !game.error}
    <div class="empty" data-testid="empty">
        <p>Choose the save files of one game to chart each player's progress day by day.</p>
        <p class="hint">Pick every save at once: one per day, all from the same map.</p>
    </div>
{/if}

<style>
    .toolbar {
        display: flex;
        align-items: center;
        gap: 12px;
        margin-bottom: 16px;
    }

    h1 {
        font-size: 22px;
        margin: 0 auto 0 0;
    }

    .status {
        color: var(--text-muted);
    }

    .empty {
        border: 1px dashed var(--border);
        border-radius: var(--radius);
        padding: 48px;
        text-align: center;
        color: var(--text-muted);
    }

    .hint {
        font-size: 13px;
    }

    .grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
        gap: 16px;
    }

    /* The charts, with the display options in a column on the right, as on a chart's page. */
    .overview {
        display: grid;
        grid-template-columns: minmax(0, 1fr) 300px;
        gap: 16px;
        align-items: start;
    }

    .overview-options {
        display: grid;
        gap: 20px;
        position: sticky;
        top: 16px;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        padding: 16px;
    }

    .card-wrap {
        position: relative;
    }

    .card-png {
        position: absolute;
        top: 9px;
        right: 10px;
        padding: 0 8px;
        line-height: 24px;
    }

    .card {
        display: grid;
        height: 100%;
        grid-template-columns: 1fr auto;
        gap: 2px 8px;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        /* Room on the right for the download button. */
        padding: 12px 52px 8px 14px;
        color: inherit;
        text-decoration: none;
    }

    .card:hover {
        border-color: var(--accent);
    }

    .card h3 {
        margin: 0;
        font-size: 15px;
    }

    .group {
        color: var(--text-muted);
        font-size: 12px;
        align-self: center;
    }

    .card-chart {
        grid-column: 1 / -1;
        height: 180px;
        /* The chart uses the full width; only the header leaves room for the button. */
        margin-right: -38px;
    }

    .detail {
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        padding: 16px;
    }

    /* All charts on the left, the stepper in the middle and Download on the right. */
    .detail-toolbar {
        display: grid;
        grid-template-columns: 1fr auto 1fr;
        align-items: center;
        gap: 8px;
        margin-bottom: 12px;
    }

    .all-charts {
        justify-self: start;
    }

    .detail-toolbar a {
        text-decoration: none;
    }

    .detail h2 {
        margin: 0;
        font-size: 18px;
    }

    .save-png {
        grid-column: 3;
        justify-self: end;
    }

    .stepper {
        display: flex;
        gap: 8px;
    }

    .detail-body {
        display: grid;
        grid-template-columns: minmax(0, 1fr) 300px;
        gap: 16px;
        margin-top: 8px;
    }

    .detail-chart {
        height: min(70vh, 640px);
    }

    .options-column {
        border-left: 1px solid var(--border);
        padding-left: 16px;
    }
</style>
