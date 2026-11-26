<script lang="ts">
    import { THEME_MODES } from "../lib/options";
    import { router } from "../lib/router.svelte";
    import { theme } from "../lib/theme.svelte";
</script>

<nav aria-label="Main">
    <div class="brand">Thieves' Guild</div>
    <ul>
        <li>
            <a href="#/saves" aria-current={router.route.page === "saves" ? "page" : undefined}>Save games</a>
        </li>
        <li>
            <span class="disabled" aria-disabled="true">Calculators <span class="badge">Coming soon</span></span>
        </li>
    </ul>
    <div class="theme">
        <span class="theme-label" id="theme-label">Theme</span>
        <div class="segments" role="group" aria-labelledby="theme-label">
            {#each THEME_MODES as mode (mode.value)}
                <button
                    aria-pressed={theme.mode === mode.value}
                    data-testid={`theme-${mode.value}`}
                    onclick={() => theme.setMode(mode.value)}
                >
                    {mode.label}
                </button>
            {/each}
        </div>
    </div>
</nav>

<style>
    nav {
        display: flex;
        flex-direction: column;
        background: var(--sidebar);
        border-right: 1px solid var(--border);
        padding: 20px 12px;
    }

    /* At the bottom of the sidebar, in view whatever the page. */
    .theme {
        position: sticky;
        bottom: 16px;
        margin-top: auto;
        padding: 16px 10px 0;
        display: grid;
        gap: 6px;
    }

    .theme-label {
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        color: var(--text-muted);
    }

    .segments {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        border: 1px solid var(--border);
        border-radius: 8px;
        overflow: hidden;
    }

    .segments button {
        border: none;
        border-radius: 0;
        padding: 5px 0;
        font-size: 13px;
        background: transparent;
    }

    .segments button + button {
        border-left: 1px solid var(--border);
    }

    .segments button[aria-pressed="true"] {
        background: var(--surface);
        font-weight: 600;
    }

    .brand {
        font-weight: 700;
        font-size: 18px;
        padding: 0 10px 18px;
    }

    ul {
        list-style: none;
        margin: 0;
        padding: 0;
        display: grid;
        gap: 2px;
    }

    a,
    .disabled {
        display: block;
        padding: 8px 10px;
        border-radius: 8px;
        color: var(--text);
        text-decoration: none;
    }

    a:hover {
        background: var(--surface);
    }

    a[aria-current="page"] {
        background: var(--surface);
        font-weight: 600;
    }

    .disabled {
        color: var(--text-muted);
        cursor: default;
    }

    .badge {
        font-size: 11px;
        border: 1px solid var(--border);
        border-radius: 999px;
        padding: 1px 7px;
        margin-left: 4px;
        white-space: nowrap;
    }
</style>
