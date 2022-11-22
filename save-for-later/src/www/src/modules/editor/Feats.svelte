<script lang="ts">
    export let c;

    import ElementList from "../ElementList.svelte";
</script>

<div class="editor-box">
    <h1 class="box-title">Feats</h1>
    {#if $c.feats.length !== 0}
        <ElementList elements={$c.feats} indices={[...$c.feats.keys()]} container={'feat'} />
    {/if}
</div>