<script lang="ts">
    export let c;

    import {render} from '../helpers.ts';
</script>

<div class="sheet-box uk-width-small">
    <h1 class="box-title">Statuses</h1>
    <dl class="uk-description-list uk-text-left">
        <dt>Defenses</dt>
        <dd>{@html $c.defenses.length?render($c.defenses.join(', ')):'None'}</dd>
        <dt>Conditions</dt>
        <dd>{@html $c.conditions.length?render($c.conditions.join(', ')):'None'}</dd>
    </dl>
</div>