<script lang="ts">
    export let c;

    import {editCharacter} from "../../state.ts";

    import ElementList from "../ElementList.svelte";

    function editAndReset(data) {
        editCharacter(data);
        document.getElementById("new-class-select").value = -1;
    }
</script>

<div class="editor-box">
    <h1 class="box-title">Classes</h1>
    {#each $c.class_names as cls, i}
        <select class="uk-select uk-width-3-4" on:change={
            editCharacter({
                class: {
                    index: i,
                    name: $c.class_choices[this.value]
                }
            })
        }>
            {#each $c.class_choices as choice, i}
                {#if cls === choice}
                    <option selected value={i}>{choice}</option>
                {:else}
                    <option value={i}>{choice}</option>
                {/if}
            {/each}
        </select><input class="uk-input uk-width-1-4" type="number" placeholder="level" value={$c.class_levels[i]}
            on:focusout={(e) => { if (e.target.value !== $c.class_levels[i]) editCharacter({
                level: {
                    index: i,
                    level: Number(e.target.value)
                }
            })}}
            on:keypress={(e) => {if (e.keyCode === 13) editCharacter({
                level: {
                    index: i,
                    level: Number(e.target.value)
                }
            })}}
        />
        <ul uk-tab class="editor-box-nav">
            <li><a>Traits</a></li>
            <li><a>Description</a></li>
        </ul>
        <ElementList elements={$c.class_features[i]} indices={[...$c.class_features[i].keys()]} container={{'class': i}} />
    {/each}
    <select class="uk-select" on:change={
        editAndReset({
            class: {
                index: $c.class_names.length,
                name: $c.class_choices[this.value],
            }
        })
    } id="new-class-select">
        <option selected value={-1}>Choose a class</option>
        {#each $c.class_choices as choice, i}
            <option value={i}>{choice}</option>
        {/each}
    </select>
</div>