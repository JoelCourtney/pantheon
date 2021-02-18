<script lang="ts">
    export let c;
</script>

<div class="sheet-box uk-width-auto">
    <h1 class="sheet-box-title-small">Passives</h1>
    <div class="uk-width-shrink">
        <div uk-grid class="uk-grid-small uk-flex-center">
            <div class="uk-text-right">
                <ul class="uk-list uk-list-collapse">
                    <li>Perception</li>
                    <li>Investigation</li>
                    <li>Insight</li>
                </ul>
            </div>
            <div class="uk-text-left">
                <ul class="uk-list uk-list-collapse">
                    <li>{c.passive_perception}</li>
                    <li>{c.passive_investigation}</li>
                    <li>{c.passive_insight}</li>
                </ul>
            </div>
        </div>
    </div>
    {#if c.passive_notes.length}
        <p class="uk-margin-remove-bottom">{@html c.passive_notes.join('<br/>')}</p>
    {/if}
</div>