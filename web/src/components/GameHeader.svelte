<script lang="ts">
    import { game } from "../lib/game.svelte";
    import { daySpan, missingDaysText } from "../lib/format";
    import type { Dataset } from "../lib/types";

    let { dataset }: { dataset: Dataset } = $props();

    const missing = $derived(missingDaysText(dataset));
    const size = $derived(`${dataset.map.size}${dataset.map.underground ? ", with underground" : ""}`);
</script>

<header>
    <div class="map">
        <h2 data-testid="map-name">{dataset.map.name}</h2>
        <span class="facts">
            <span>Map size {size}</span>
            <span data-testid="day-span">{daySpan(dataset)}</span>
            {#if missing}<span class="missing" data-testid="missing-days">{missing}</span>{/if}
        </span>
    </div>
    <div class="players" role="group" aria-label="Players shown in the charts">
        {#each dataset.players as player (player.id)}
            {@const shown = !game.hidden.has(player.id)}
            <button
                class="chip"
                class:off={!shown}
                aria-pressed={shown}
                data-testid={`chip-${player.id}`}
                title={shown ? `Hide ${player.name}` : `Show ${player.name}`}
                onclick={() => game.togglePlayer(player.id)}
            >
                <span class="dot" style:background={player.color}></span>
                {player.name}
                {#if player.eliminatedDay !== null}<span class="out">out on day {player.eliminatedDay}</span>{/if}
            </button>
        {/each}
    </div>
</header>

<style>
    header {
        display: grid;
        gap: 10px;
        margin-bottom: 16px;
    }

    h2 {
        margin: 0;
        font-size: 18px;
    }

    .facts {
        display: flex;
        flex-wrap: wrap;
        gap: 4px 16px;
        color: var(--text-muted);
        font-size: 13px;
    }

    .missing {
        background: var(--warning-bg);
        color: var(--text);
        border-radius: 999px;
        padding: 0 8px;
    }

    .players {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
    }

    .chip {
        display: inline-flex;
        align-items: center;
        gap: 6px;
        border-radius: 999px;
        padding: 3px 12px 3px 8px;
    }

    .chip.off {
        opacity: 0.45;
        text-decoration: line-through;
    }

    .dot {
        width: 12px;
        height: 12px;
        border-radius: 50%;
    }

    .out {
        color: var(--text-muted);
        font-size: 12px;
    }
</style>
