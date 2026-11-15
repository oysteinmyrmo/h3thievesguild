<script lang="ts">
    import Dialog from "./Dialog.svelte";
    import { splitMessage } from "../lib/message";

    let { message, onanswer }: { message: string; onanswer: (proceed: boolean) => void } = $props();
    const parts = $derived(splitMessage(message));
</script>

<Dialog title="Some days have no save" testid="missing-days-dialog" oncancel={() => onanswer(false)}>
    <p>{parts.title}</p>
    <ul>
        {#each parts.items as item (item)}<li>{item}</li>{/each}
    </ul>
    {#each parts.after as line (line)}<p>{line}</p>{/each}
    <div class="buttons">
        <button onclick={() => onanswer(false)}>Cancel</button>
        <button class="primary" onclick={() => onanswer(true)}>Continue</button>
    </div>
</Dialog>
