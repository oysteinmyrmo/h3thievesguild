<script lang="ts">
    import { untrack } from "svelte";
    import Dialog from "./Dialog.svelte";
    import type { PlayerNames } from "../lib/names";
    import type { PlayerSeries } from "../lib/wasm/pkg/h3wasm";

    let {
        players,
        names,
        onsave,
        oncancel,
    }: {
        players: PlayerSeries[];
        names: PlayerNames;
        onsave: (names: Record<string, string>) => void;
        oncancel: () => void;
    } = $props();

    // A copy of the names when the dialog opens, edited here and handed over only on Save.
    let draft = $state<Record<string, string>>(
        untrack(() => Object.fromEntries(players.map((p) => [p.id, names[p.id] ?? ""]))),
    );
</script>

<Dialog title="Name players" testid="name-players-dialog" {oncancel}>
    <form
        onsubmit={(event) => {
            event.preventDefault();
            onsave({ ...draft });
        }}
    >
        <p class="hint">Charts show these names. Leave a name empty to use the colour.</p>
        {#each players as player (player.id)}
            <label>
                <span class="dot" style:background={player.color}></span>
                <span class="colour">{player.name}</span>
                <input
                    type="text"
                    bind:value={draft[player.id]}
                    placeholder={player.name}
                    data-testid={`name-${player.id}`}
                    autocomplete="off"
                    spellcheck="false"
                />
            </label>
        {/each}
        <div class="buttons">
            <button type="button" onclick={oncancel}>Cancel</button>
            <button type="submit" class="primary">Save</button>
        </div>
    </form>
</Dialog>

<style>
    .hint,
    .colour {
        color: var(--text-muted);
    }

    .hint {
        font-size: 13px;
    }

    label {
        display: grid;
        grid-template-columns: 12px 64px 1fr;
        align-items: center;
        gap: 10px;
        margin: 8px 0;
    }

    .dot {
        width: 12px;
        height: 12px;
        border-radius: 50%;
    }

    input {
        font: inherit;
        color: var(--text);
        background: var(--bg);
        border: 1px solid var(--border);
        border-radius: 6px;
        padding: 5px 8px;
    }
</style>
