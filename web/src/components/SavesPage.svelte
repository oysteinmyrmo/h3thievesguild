<script lang="ts">
    import ErrorPanel from "./ErrorPanel.svelte";
    import GameHeader from "./GameHeader.svelte";
    import MissingDaysDialog from "./MissingDaysDialog.svelte";
    import StatChart from "./StatChart.svelte";
    import { game } from "../lib/game.svelte";
    import { loadSaves, readFiles } from "../lib/load";
    import { X_AXIS_MODES, type XAxisMode } from "../lib/options";
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
        <section class="detail" aria-label={detail.name}>
            <div class="detail-bar">
                <a class="button" href="#/saves">← All charts</a>
                <h2>{detail.name}</h2>
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
            </div>
            <div class="detail-body">
                <div class="detail-chart">
                    <StatChart
                        dataset={game.dataset}
                        stat={detail}
                        hidden={game.hidden}
                        names={game.names}
                        compact={false}
                        xAxis={game.options.xAxis}
                    />
                </div>
                <aside class="options" aria-label="Chart options">
                    <h3>Display</h3>
                    <label>
                        X-axis
                        <select
                            data-testid="x-axis"
                            value={game.options.xAxis}
                            onchange={(e) => game.setOptions({ xAxis: e.currentTarget.value as XAxisMode })}
                        >
                            {#each X_AXIS_MODES as mode (mode.value)}
                                <option value={mode.value}>{mode.label}</option>
                            {/each}
                        </select>
                    </label>
                </aside>
            </div>
        </section>
    {:else}
        <section class="grid" aria-label="All charts">
            {#each game.dataset.playerStats as info (info.id)}
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
                            xAxis={game.options.xAxis}
                        />
                    </div>
                </a>
            {/each}
        </section>
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

    .card {
        display: grid;
        grid-template-columns: 1fr auto;
        gap: 2px 8px;
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        padding: 12px 14px 8px;
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
    }

    .detail {
        background: var(--surface);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        padding: 16px;
    }

    .detail-bar {
        display: flex;
        align-items: center;
        gap: 16px;
    }

    .detail-bar h2 {
        margin: 0;
        font-size: 18px;
    }

    .detail-bar a {
        text-decoration: none;
    }

    .stepper {
        display: flex;
        gap: 8px;
        margin-left: auto;
    }

    .detail-body {
        display: grid;
        grid-template-columns: minmax(0, 1fr) 220px;
        gap: 16px;
        margin-top: 8px;
    }

    .detail-chart {
        height: min(70vh, 640px);
    }

    .options {
        border-left: 1px solid var(--border);
        padding-left: 16px;
    }

    .options h3 {
        margin: 0 0 12px;
        font-size: 13px;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        color: var(--text-muted);
    }

    .options label {
        display: grid;
        gap: 4px;
        font-size: 13px;
    }

    .options select {
        font: inherit;
        padding: 5px 8px;
        border: 1px solid var(--border);
        border-radius: 8px;
        background: var(--surface);
        color: var(--text);
    }
</style>
