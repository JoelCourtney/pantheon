<script lang="ts">
    export let c;

    import {updateField} from "../../state";
</script>

<div class="editor-box">
    <h1 class="box-title">Description</h1>
    <textarea class="uk-textarea uk-height-large" placeholder="Insert tragic and edgy backstory here." value={$c.description}
              on:focusout={(e) => updateField($c, "description", e.target.value)}></textarea>
</div>