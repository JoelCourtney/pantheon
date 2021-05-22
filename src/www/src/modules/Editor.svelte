<script lang="ts">
    import ElementList from "./ElementList.svelte";

    export let c;

    import {editCharacter} from '../state.ts';

    function updateField(field, value) {
        if (value !== $c[field]) {
            let request: any = {};
            request[field] = value;
            editCharacter(request);
        }
    }
</script>

<div class="uk-grid-row-small uk-grid-column-collapse uk-child-width-2-3 uk-flex-center" uk-grid>
    <div class="builder-box">
        <h1 class="box-title">Name</h1>
        <input class="uk-input" placeholder="Person McPersonFace" value={$c.name}
               on:focusout={(e) => {updateField("name", e.target.value)}}
               on:keypress={(e) => {if (e.keyCode === 13) updateField("name", e.target.value)}}/>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Abilities</h1>
        <div class="uk-grid-small" uk-grid>
            <div class="uk-width-1-3">
                <input class="uk-input" type="number" placeholder="STR" value={$c.base_abilities.strength}
                       on:focusout={(e) => updateField("ability_score", ["Strength", Number(e.target.value)])}
                       on:keypress={(e) => {if (e.keyCode === 13) updateField("ability_score", ["Strength", Number(e.target.value)])}}
                />
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="number" placeholder="DEX" value={$c.base_abilities.dexterity}
                       on:focusout={(e) => updateField("ability_score", ["Dexterity", Number(e.target.value)])}
                       on:keypress={(e) => {if (e.keyCode === 13) updateField("ability_score", ["Dexterity", Number(e.target.value)])}}
                />
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="number" placeholder="CON" value={$c.base_abilities.constitution}
                       on:focusout={(e) => updateField("ability_score", ["Constitution", Number(e.target.value)])}
                       on:keypress={(e) => {if (e.keyCode === 13) updateField("ability_score", ["Constitution", Number(e.target.value)])}}
                />
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="number" placeholder="INT" value={$c.base_abilities.intelligence}
                       on:focusout={(e) => updateField("ability_score", ["Intelligence", Number(e.target.value)])}
                       on:keypress={(e) => {if (e.keyCode === 13) updateField("ability_score", ["Intelligence", Number(e.target.value)])}}
                />
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="number" placeholder="WIS" value={$c.base_abilities.wisdom}
                       on:focusout={(e) => updateField("ability_score", ["Wisdom", Number(e.target.value)])}
                       on:keypress={(e) => {if (e.keyCode === 13) updateField("ability_score", ["Wisdom", Number(e.target.value)])}}
                />
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="number" placeholder="CHA" value={$c.base_abilities.charisma}
                       on:focusout={(e) => updateField("ability_score", ["Charisma", Number(e.target.value)])}
                       on:keypress={(e) => {if (e.keyCode === 13) updateField("ability_score", ["Charisma", Number(e.target.value)])}}
                />
            </div>
        </div>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Race</h1>
        <select class="uk-select" on:change={
            editCharacter({
                race: $c.race_choices[this.value]
            })
        }>
            {#if $c.race_name === 'Unknown'}
                <option selected value={-1}>Choose a race</option>
            {/if}
            {#each $c.race_choices as race, i}
                {#if $c.race_name === race}
                    <option selected value={i}>{race}</option>
                {:else}
                    <option value={i}>{race}</option>
                {/if}
            {/each}
        </select>
        {#if $c.race_traits.length !== 0}
            <ul uk-tab class="builder-box-nav">
                <li><a>Traits</a></li>
                <li><a>Description</a></li>
            </ul>
            <ElementList elements={$c.race_traits} indices={[...$c.race_traits.keys()]} container={'race'}/>
        {/if}
    </div>
    <div class="builder-box">
        <h1 class="box-title">Classes</h1>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Background</h1>
        <select class="uk-select" on:change={
            editCharacter({
                background: $c.background_choices[this.value]
            })
        }>
            {#if $c.background_name === 'Unknown'}
                <option selected value={-1}>Choose a background</option>
            {/if}
            {#each $c.background_choices as background, i}
                {#if $c.background_name === background}
                    <option selected value={i}>{background}</option>
                {:else}
                    <option value={i}>{background}</option>
                {/if}
            {/each}
        </select>
        {#if $c.background_features.length !== 0}
            <ul uk-tab class="builder-box-nav">
                <li><a>Features</a></li>
                <li><a>Description</a></li>
            </ul>
            <ElementList elements={$c.background_features} indices={[...$c.background_features.keys()]} container={'background'}/>
        {/if}
    </div>
    <div class="builder-box">
        <h1 class="box-title">Feats</h1>
        {#if $c.feats.length !== 0}
            <ElementList elements={$c.feats} indices={[...$c.feats.keys()]} container={'feat'} />
        {/if}
    </div>
    <div class="builder-box">
        <h1 class="box-title">Description</h1>
        <textarea class="uk-textarea uk-height-large" placeholder="Insert tragic and edgy backstory here." value={$c.description}
            on:focusout={(e) => updateField("description", e.target.value)}></textarea>
    </div>
</div>