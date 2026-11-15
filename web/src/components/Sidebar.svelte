<script lang="ts">
    import { router } from "../lib/router.svelte";
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
</nav>

<style>
    nav {
        background: var(--sidebar);
        border-right: 1px solid var(--border);
        padding: 20px 12px;
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
