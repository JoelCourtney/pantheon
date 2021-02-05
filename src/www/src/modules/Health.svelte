<script lang="ts">
    export let c;
    export let view;
</script>

<div class="sheet-box uk-width-auto">
        <div class="uk-display-inline-block uk-margin-right">
            <h1 class="sheet-box-title-small">Health</h1>
            <p class="sheet-box-number">{c.health}</p>
        </div>
        <div class="uk-display-inline-block uk-margin-right">
            <h1 class="sheet-box-title-small">Max</h1>
            <p class="sheet-box-number">{c.max_health}</p>
        </div>
        <div class="uk-display-inline-block">
            <h1 class="sheet-box-title-small">Temp</h1>
            <p class="sheet-box-number">{c.temp_health}</p>
        </div>
</div>