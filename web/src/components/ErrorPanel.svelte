<script lang="ts">
    import { errorView } from "../lib/errors";
    import type { LoadError } from "../lib/types";

    let { error }: { error: LoadError } = $props();
    const view = $derived(errorView(error));
</script>

<div class="error" role="alert" data-testid="error" data-kind={error.kind}>
    <strong>{view.title}</strong>
    {#if view.items.length > 0}
        <ul>
            {#each view.items as item (item)}<li>{item}</li>{/each}
        </ul>
    {/if}
    {#if view.hint}<p>{view.hint}</p>{/if}
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
        margin: 8px 0;
        padding-left: 20px;
    }

    p {
        margin: 0;
    }
</style>
