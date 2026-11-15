<script lang="ts">
    import Sidebar from "./components/Sidebar.svelte";
    import SavesPage from "./components/SavesPage.svelte";
    import CalculatorsPage from "./components/CalculatorsPage.svelte";
    import { game } from "./lib/game.svelte";
    import { router } from "./lib/router.svelte";

    game.restore();
</script>

<div class="layout">
    <Sidebar />
    <main>
        {#if router.route.page === "calculators"}
            <CalculatorsPage />
        {:else}
            <SavesPage stat={router.route.stat} />
        {/if}
    </main>
</div>

<style>
    .layout {
        display: grid;
        grid-template-columns: 220px 1fr;
        min-height: 100vh;
        min-width: 960px;
    }

    main {
        padding: 24px 32px 48px;
        min-width: 0;
    }
</style>
