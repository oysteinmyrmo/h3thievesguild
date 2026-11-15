<script lang="ts">
    import { onMount } from "svelte";

    let { message, onanswer }: { message: string; onanswer: (proceed: boolean) => void } = $props();

    let dialog: HTMLDialogElement;
    // The message lists the missing days after its first line and asks to continue on its last.
    const lines = $derived(message.split("\n"));

    onMount(() => dialog.showModal());

    function answer(proceed: boolean) {
        dialog.close();
        onanswer(proceed);
    }
</script>

<dialog bind:this={dialog} data-testid="missing-days-dialog" oncancel={() => onanswer(false)}>
    <h2>Some days have no save</h2>
    <p>{lines[0]}</p>
    <ul>
        {#each lines.slice(1, -1) as line (line)}<li>{line.replace(/^- /, "")}</li>{/each}
    </ul>
    <p>{lines[lines.length - 1]}</p>
    <div class="buttons">
        <button onclick={() => answer(false)}>Cancel</button>
        <button class="primary" onclick={() => answer(true)}>Continue</button>
    </div>
</dialog>

<style>
    dialog {
        background: var(--surface);
        color: var(--text);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        max-width: 520px;
        padding: 20px 24px;
    }

    dialog::backdrop {
        background: rgba(0, 0, 0, 0.4);
    }

    h2 {
        margin-top: 0;
        font-size: 18px;
    }

    .buttons {
        display: flex;
        justify-content: flex-end;
        gap: 8px;
    }
</style>
