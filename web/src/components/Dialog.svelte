<script lang="ts">
    // A modal dialog, open while it is shown. Escape calls `oncancel`; the parent closes the
    // dialog by no longer showing it.
    import { onMount, type Snippet } from "svelte";

    let {
        title,
        testid,
        oncancel,
        children,
    }: { title: string; testid: string; oncancel: () => void; children: Snippet } = $props();

    let dialog: HTMLDialogElement;
    onMount(() => dialog.showModal());
</script>

<dialog bind:this={dialog} data-testid={testid} oncancel={oncancel}>
    <h2>{title}</h2>
    {@render children()}
</dialog>

<style>
    dialog {
        background: var(--surface);
        color: var(--text);
        border: 1px solid var(--border);
        border-radius: var(--radius);
        width: min(520px, 90vw);
        padding: 20px 24px;
    }

    dialog::backdrop {
        background: rgba(0, 0, 0, 0.4);
    }

    h2 {
        margin-top: 0;
        font-size: 18px;
    }

    dialog :global(.buttons) {
        display: flex;
        justify-content: flex-end;
        gap: 8px;
        margin-top: 16px;
    }
</style>
