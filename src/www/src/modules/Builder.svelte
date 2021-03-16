<script lang="ts">
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
            {#if $c.race_name === 'Unknown Race'}
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
        <ul uk-tab class="builder-box-nav">
            <li><a>Traits</a></li>
            <li><a>Description</a></li>
        </ul>
        <ul class="uk-list uk-list-bullet uk-margin-remove-top">
            {#each $c.race_traits as trait, i}
                <li>
                    {@html render(trait[0])}
                    {#if trait[1] !== null}
                        {#each trait[1].current_choices as current, j}
                            <select class="uk-select" on:change={
                                editCharacter({
                                    feature: {
                                        container: 'race',
                                        feature_index: i,
                                        choice_index: j,
                                        choice: $c.race_traits[i][1].all_choices[j][this.value]
                                    }
                                })
                            }>
                                {#if current === 'Unknown'}
                                    <option selected value={-1}>Choose</option>
                                {/if}
                                {#each trait[1].all_choices[j] as choice, k}
                                    {#if current !== choice}
                                        <option value={k}>{choice}</option>
                                    {:else}
                                        <option selected value={k}>{choice}</option>
                                    {/if}
                                {/each}
                            </select>
                        {/each}
                    {/if}
                </li>
            {/each}
        </ul>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Classes</h1>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Background</h1>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Description</h1>
        <textarea class="uk-textarea uk-height-large" placeholder="Insert tragic and edgy backstory here." value={$c.description}
            on:focusout={(e) => updateText("description", e.target.value)}/>
    </div>
</div>