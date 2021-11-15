<script lang="ts">
    export let c;

    import {updateField} from "../../state.ts";
</script>
<div class="editor-box">
    <h1 class="box-title">Abilities</h1>
    <div class="uk-grid-small" uk-grid>
        <div class="uk-width-1-3">
            <input class="uk-input" type="number" placeholder="STR" value={$c.base_abilities.strength}
                   on:focusout={(e) => updateField($c, "ability_score", ["Strength", Number(e.target.value)])}
                   on:keypress={(e) => {if (e.keyCode === 13) updateField($c, "ability_score", ["Strength", Number(e.target.value)])}}
            />
        </div>
        <div class="uk-width-1-3">
            <input class="uk-input" type="number" placeholder="DEX" value={$c.base_abilities.dexterity}
                   on:focusout={(e) => updateField($c, "ability_score", ["Dexterity", Number(e.target.value)])}
                   on:keypress={(e) => {if (e.keyCode === 13) updateField($c, "ability_score", ["Dexterity", Number(e.target.value)])}}
            />
        </div>
        <div class="uk-width-1-3">
            <input class="uk-input" type="number" placeholder="CON" value={$c.base_abilities.constitution}
                   on:focusout={(e) => updateField($c, "ability_score", ["Constitution", Number(e.target.value)])}
                   on:keypress={(e) => {if (e.keyCode === 13) updateField($c, "ability_score", ["Constitution", Number(e.target.value)])}}
            />
        </div>
        <div class="uk-width-1-3">
            <input class="uk-input" type="number" placeholder="INT" value={$c.base_abilities.intelligence}
                   on:focusout={(e) => updateField($c, "ability_score", ["Intelligence", Number(e.target.value)])}
                   on:keypress={(e) => {if (e.keyCode === 13) updateField($c, "ability_score", ["Intelligence", Number(e.target.value)])}}
            />
        </div>
        <div class="uk-width-1-3">
            <input class="uk-input" type="number" placeholder="WIS" value={$c.base_abilities.wisdom}
                   on:focusout={(e) => updateField($c, "ability_score", ["Wisdom", Number(e.target.value)])}
                   on:keypress={(e) => {if (e.keyCode === 13) updateField($c, "ability_score", ["Wisdom", Number(e.target.value)])}}
            />
        </div>
        <div class="uk-width-1-3">
            <input class="uk-input" type="number" placeholder="CHA" value={$c.base_abilities.charisma}
                   on:focusout={(e) => updateField($c, "ability_score", ["Charisma", Number(e.target.value)])}
                   on:keypress={(e) => {if (e.keyCode === 13) updateField($c, "ability_score", ["Charisma", Number(e.target.value)])}}
            />
        </div>
    </div>
</div>