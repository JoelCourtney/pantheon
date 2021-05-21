<script lang="ts">
    export let elements;
    export let container;

    import {editCharacter} from "../state";
    import {render} from "../helpers";
</script>

<ul class="uk-list uk-list-bullet uk-margin-remove-top">
    {#each elements as element, i}
        <li>
            {@html render(element.text)}
            {#if element.type === 'choice'}
                {#each element.data.current_choices as current, j}
                    <select class="uk-select" on:change={
                        editCharacter({
                            choice: {
                                container: container,
                                element_index: i,
                                choice_index: j,
                                choice: elements[i].data.all_choices[this.value]
                            }
                        })
                    }>
                        {#if current === 'Unknown'}
                            <option selected value={-1}>Choose</option>
                        {/if}
                        {#each element.data.all_choices as choice, k}
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