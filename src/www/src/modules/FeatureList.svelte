<script lang="ts">
    export let features;
    export let container;

    import {editCharacter} from "../state";
    import {render} from "../helpers";

    const field = (container==='race')?'race_traits':'';

</script>

<ul class="uk-list uk-list-bullet uk-margin-remove-top">
    {#each features as feature, i}
        <li>
            {@html render(feature[0])}
            {#if feature[1] !== null}
                {#each feature[1].current_choices as current, j}
                    <select class="uk-select" on:change={
                        editCharacter({
                            feature: {
                                container: container,
                                feature_index: i,
                                choice_index: j,
                                choice: features[i][1].all_choices[j][this.value]
                            }
                        })
                    }>
                        {#if current === 'Unknown'}
                            <option selected value={-1}>Choose</option>
                        {/if}
                        {#each feature[1].all_choices[j] as choice, k}
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