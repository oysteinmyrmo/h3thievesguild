<script lang="ts">
    import { onMount, untrack } from "svelte";
    import type { PlayerNames } from "../lib/names";
    import type { PlayerSeries } from "../lib/types";

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

    let dialog: HTMLDialogElement;
    // A copy of the names when the dialog opens, edited here and handed over only on Save.
    let draft = $state<Record<string, string>>(
        untrack(() => Object.fromEntries(players.map((p) => [p.id, names[p.id] ?? ""]))),
    );

    onMount(() => dialog.showModal());

    function save(event: SubmitEvent) {
        event.preventDefault();
        dialog.close();
        onsave({ ...draft });
    }

    function cancel() {
        dialog.close();
        oncancel();
    }
</script>

<dialog bind:this={dialog} data-testid="name-players-dialog" oncancel={oncancel}>
    <form onsubmit={save}>
        <h2>Name players</h2>
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
            <button type="button" onclick={cancel}>Cancel</button>
            <button type="submit" class="primary">Save</button>
        </div>
    </form>
</dialog>

<style>
    dialog {
        background: var(--surface);
        color: var(--text);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        width: 420px;
        padding: 20px 24px;
    }

    dialog::backdrop {
        background: rgba(0, 0, 0, 0.4);
    }

    h2 {
        margin-top: 0;
        font-size: 18px;
    }

    .hint {
        color: var(--text-muted);
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

    .colour {
        color: var(--text-muted);
    }

    input {
        font: inherit;
        color: var(--text);
        background: var(--bg);
        border: 1px solid var(--border);
        border-radius: 6px;
        padding: 5px 8px;
    }

    .buttons {
        display: flex;
        justify-content: flex-end;
        gap: 8px;
        margin-top: 16px;
    }
</style>
