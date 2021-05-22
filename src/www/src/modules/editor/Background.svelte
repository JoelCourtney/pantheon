<script lang="ts">
    export let c;

    import {editCharacter} from "../../state";
    import ElementList from "../ElementList.svelte";
</script>
<div class="editor-box">
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
        <ul uk-tab class="editor-box-nav">
            <li><a>Features</a></li>
            <li><a>Description</a></li>
        </ul>
        <ElementList elements={$c.background_features} indices={[...$c.background_features.keys()]} container={'background'}/>
    {/if}
</div>