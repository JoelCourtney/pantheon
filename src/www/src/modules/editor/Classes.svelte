<script lang="ts">
    export let c;

    import {editCharacter} from "../../state.ts";

    console.log($c.class_names);
    console.log($c.total_level);

    function editAndReset(data) {
        editCharacter(data);
        document.getElementById("new-class-select").value = -1;
    }
</script>

<div class="editor-box">
    <h1 class="box-title">Classes</h1>
    {#each $c.class_names as cls, i}
        <select class="uk-select" on:change={
            editCharacter({
                class: {
                    index: i,
                    name: $c.class_choices[this.value],
                    level: 1
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
        </select>
    {/each}
    <select class="uk-select" on:change={
        editAndReset({
            class: {
                index: $c.class_names.length,
                name: $c.class_choices[this.value],
                level: 1
            }
        })
    } id="new-class-select">
        <option selected value={-1}>Choose a class</option>
        {#each $c.class_choices as choice, i}
            <option value={i}>{choice}</option>
        {/each}
    </select>
</div>