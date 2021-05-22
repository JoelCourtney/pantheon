<script lang="ts">
    import { signedInt } from '../../helpers.ts';

    export let c;
</script>

<div class="sheet-box uk-width-large">
    <div uk-grid class="uk-child-width-1-3 uk-grid-row-medium">
        <div>
            <h1 class="box-title uk-margin-remove">Strength</h1>
            <p class="sheet-box-number uk-margin-small">{$c.abilities.strength}|{signedInt($c.ability_modifiers.strength)}</p>
        </div>
        <div>
            <h1 class="box-title">Dexterity</h1>
            <p class="sheet-box-number uk-margin-small">{$c.abilities.dexterity}|{signedInt($c.ability_modifiers.dexterity)}</p>
        </div>
        <div>
            <h1 class="box-title">Constitution</h1>
            <p class="sheet-box-number uk-margin-small">{$c.abilities.constitution}|{signedInt($c.ability_modifiers.constitution)}</p>
        </div>
        <div>
            <p class="sheet-box-number">{$c.abilities.intelligence}|{signedInt($c.ability_modifiers.intelligence)}</p>
            <h1 class="box-title uk-margin-remove">Intelligence</h1>
        </div>
        <div>
            <p class="sheet-box-number">{$c.abilities.wisdom}|{signedInt($c.ability_modifiers.wisdom)}</p>
            <h1 class="box-title uk-margin-remove">Wisdom</h1>
        </div>
        <div>
            <p class="sheet-box-number">{$c.abilities.charisma}|{signedInt($c.ability_modifiers.charisma)}</p>
            <h1 class="box-title uk-margin-remove">Charisma</h1>
        </div>
    </div>
</div>