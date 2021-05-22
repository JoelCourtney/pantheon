<script lang="ts">
    export let c;

    import {editCharacter} from "../../state";

    import ElementList from "../ElementList.svelte";
</script>

<div class="editor-box">
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
        <ul uk-tab class="editor-box-nav">
            <li><a>Traits</a></li>
            <li><a>Description</a></li>
        </ul>
        <ElementList elements={$c.race_traits} indices={[...$c.race_traits.keys()]} container={'race'}/>
    {/if}
</div>