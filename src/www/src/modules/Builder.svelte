<script lang="ts">
    export let c;

    import {render} from "../helpers.ts";
    import {editCharacter} from '../state.ts';
</script>

<div class="uk-grid-small uk-child-width-2-3 uk-flex-center" uk-grid>
    <div class="builder-box">
        <h1 class="box-title">Name</h1>
        <input class="uk-input" placeholder="Person McPersonFace"/>
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
        <select class="uk-select">
            <option>Hooman</option>
            <option>Elv</option>
            <option>Dwarv</option>
        </select>
        <ul uk-tab class="builder-box-nav">
            <li><a>Traits</a></li>
            <li><a>Description</a></li>
        </ul>
        <ul class="uk-list uk-list-bullet uk-margin-remove-top">
            {#each c.race_traits as trait, i}
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
                                        choice: c.race_traits[i][1].all_choices[j][this.value]
                                    }
                                })
                            }>
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
        <textarea class="uk-textarea uk-height-large" placeholder="Insert tragic and edgy backstory here."></textarea>
    </div>
</div>