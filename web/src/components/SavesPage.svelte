<script lang="ts">
    import ErrorPanel from "./ErrorPanel.svelte";
    import GameHeader from "./GameHeader.svelte";
    import MissingDaysDialog from "./MissingDaysDialog.svelte";
    import StatChart from "./StatChart.svelte";
    import { game } from "../lib/game.svelte";
    import { loadSaves, readFiles } from "../lib/loader";
    import { router } from "../lib/router.svelte";
    import type { Dataset } from "../lib/types";

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

    const detail = $derived(game.dataset && stat ? game.dataset.playerStats.find((s) => s.id === stat) : undefined);
</script>

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
            </div>
            <div class="detail-chart">
                <StatChart dataset={game.dataset} stat={detail} hidden={game.hidden} compact={false} />
            </div>
        </section>
    {:else}
        <section class="grid" aria-label="All charts">
            {#each game.dataset.playerStats as info (info.id)}
                <a class="card" href={`#/saves/${info.id}`} data-testid={`card-${info.id}`}>
                    <h3>{info.name}</h3>
                    <span class="group">{info.group}</span>
                    <div class="card-chart">
                        <StatChart dataset={game.dataset} stat={info} hidden={game.hidden} compact={true} />
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

    .detail-chart {
        height: min(70vh, 640px);
        margin-top: 8px;
    }
</style>
