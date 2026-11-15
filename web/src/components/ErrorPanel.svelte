<script lang="ts">
    import type { LoadError } from "../lib/load";
    import { splitMessage } from "../lib/message";

    let { error }: { error: LoadError } = $props();
    // The parser's message names every file involved.
    const parts = $derived(splitMessage(error.message));
</script>

<div class="error" role="alert" data-testid="error" data-kind={error.kind}>
    <strong>{parts.title}</strong>
    {#if parts.items.length > 0}
        <ul>
            {#each parts.items as item (item)}<li>{item}</li>{/each}
        </ul>
    {/if}
    {#each parts.after as line (line)}<p>{line}</p>{/each}
</div>

<style>
    .error {
        background: var(--danger-bg);
        border: 1px solid var(--danger);
        border-radius: var(--radius);
        padding: 12px 16px;
        margin-bottom: 16px;
    }

    ul {
        margin: 8px 0 0;
        padding-left: 20px;
    }

    p {
        margin: 8px 0 0;
    }
</style>
