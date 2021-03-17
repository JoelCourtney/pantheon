<script lang="ts">
    import FeatureList from "./FeatureList.svelte";

    export let c;

    import {render} from "../helpers.ts";
    import {editCharacter} from '../state.ts';

    function updateText(field, text) {
        if (text !== $c[field]) {
            let request: any = {};
            request[field] = text;
            editCharacter(request);
        }
    }
</script>

<div class="uk-grid-small uk-child-width-2-3 uk-flex-center" uk-grid>
    <div class="builder-box">
        <h1 class="box-title">Name</h1>
        <input class="uk-input" placeholder="Person McPersonFace" value={$c.name}
               on:focusout={(e) => {updateText("name", e.target.value)}}
               on:keypress={(e) => {if (e.keyCode === 13) updateText("name", e.target.value)}}/>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Abilities</h1>
        <div class="uk-grid-small" uk-grid>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="STR">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="DEX">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="CON">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="INT">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="WIS">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="CHA">
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
            <FeatureList features={$c.race_traits} container={'race'} />
        {/if}
    </div>
    <div class="builder-box">
        <h1 class="box-title">Classes</h1>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Background</h1>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Feats</h1>
        {#if $c.feats.length !== 0}
            <FeatureList features={$c.feats} container={'feat'} />
        {/if}
    </div>
    <div class="builder-box">
        <h1 class="box-title">Description</h1>
        <textarea class="uk-textarea uk-height-large" placeholder="Insert tragic and edgy backstory here." value={$c.description}
            on:focusout={(e) => updateText("description", e.target.value)}/>
    </div>
</div>