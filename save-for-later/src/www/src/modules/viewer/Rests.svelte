<script lang="ts">
    export let c;

    import {editCharacter} from "../../state";
</script>

<div class="sheet-box">
    <h1 class="box-title">Rest</h1>
    <button class="uk-button uk-button-default uk-margin-remove-right" type="button" on:click={
        () => editCharacter({
            event: "ShortRest"
        })
    }>Short</button>
    <button class="uk-button uk-button-default uk-margin-remove-left" type="button" on:click={
        () => editCharacter({
            event: "LongRest"
        })
    }>Long</button>
</div>