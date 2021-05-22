<script lang="ts">
    export let c;

    import {updateField} from "../../state.ts";
</script>

<div class="editor-box">
    <h1 class="box-title">Name</h1>
    <input class="uk-input" placeholder="Person McPersonFace" value={$c.name}
           on:focusout={(e) => {updateField($c, "name", e.target.value)}}
           on:keypress={(e) => {if (e.keyCode === 13) updateField($c, "name", e.target.value)}}/>
</div>