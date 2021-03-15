<script lang="ts">
    export let c;
</script>

<div class="sheet-box uk-width-small">
    <h1 class="box-title">Statuses</h1>
    <dl class="uk-description-list uk-text-left">
        <dt>Defenses</dt>
        <dd>{c.defenses.length?c.statuses.join(', '):'None'}</dd>
        <dt>Conditions</dt>
        <dd>{c.conditions.length?c.conditions.join(', '):'None'}</dd>
    </dl>
</div>