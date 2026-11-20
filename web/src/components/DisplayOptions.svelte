<script lang="ts">
    import { game } from "../lib/game.svelte";
    import { CHART_TYPES, X_AXIS_MODES, type ChartType, type XAxisMode } from "../lib/options";
</script>

<div class="options" role="group" aria-label="Display options">
    <h3>Display options</h3>
    <label>
        Chart
        <select
            data-testid="chart-type"
            value={game.options.chartType}
            onchange={(e) => game.setOptions({ chartType: e.currentTarget.value as ChartType })}
        >
            {#each CHART_TYPES as type (type.value)}
                <option value={type.value}>{type.label}</option>
            {/each}
        </select>
    </label>
    <label>
        X-axis
        <select
            data-testid="x-axis"
            value={game.options.xAxis}
            onchange={(e) => game.setOptions({ xAxis: e.currentTarget.value as XAxisMode })}
        >
            {#each X_AXIS_MODES as mode (mode.value)}
                <option value={mode.value}>{mode.label}</option>
            {/each}
        </select>
    </label>
</div>

<style>
    .options {
        display: grid;
        gap: 12px;
        font-size: 13px;
    }

    h3 {
        margin: 0;
        font-size: 13px;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        color: var(--text-muted);
    }

    label {
        display: grid;
        gap: 4px;
    }

    select {
        font: inherit;
        padding: 5px 8px;
        border: 1px solid var(--border);
        border-radius: 8px;
        background: var(--surface);
        color: var(--text);
    }
</style>
