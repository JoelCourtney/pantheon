<script lang="ts">
    import { signedInt } from '../helpers.ts';

    export let c;
</script>

<div class="sheet-box uk-width-large">
    <div uk-grid class="uk-child-width-1-3 uk-grid-row-medium">
        <div>
            <h1 class="box-title uk-margin-remove">Strength</h1>
            <p class="sheet-box-number uk-margin-small">{c.strength}|{signedInt(c.strength_modifier)}</p>
        </div>
        <div>
            <h1 class="box-title">Dexterity</h1>
            <p class="sheet-box-number uk-margin-small">{c.dexterity}|{signedInt(c.dexterity_modifier)}</p>
        </div>
        <div>
            <h1 class="box-title">Constitution</h1>
            <p class="sheet-box-number uk-margin-small">{c.constitution}|{signedInt(c.constitution_modifier)}</p>
        </div>
        <div>
            <p class="sheet-box-number">{c.intelligence}|{signedInt(c.intelligence_modifier)}</p>
            <h1 class="box-title uk-margin-remove">Intelligence</h1>
        </div>
        <div>
            <p class="sheet-box-number">{c.wisdom}|{signedInt(c.wisdom_modifier)}</p>
            <h1 class="box-title uk-margin-remove">Wisdom</h1>
        </div>
        <div>
            <p class="sheet-box-number">{c.charisma}|{signedInt(c.charisma_modifier)}</p>
            <h1 class="box-title uk-margin-remove">Charisma</h1>
        </div>
    </div>
</div>