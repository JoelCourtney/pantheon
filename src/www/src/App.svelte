<script lang='ts'>
    import './main.css';

    import {c, view} from './state.ts';

    import Sidebar from "./modules/Sidebar.svelte";
    import Viewer from "./modules/Viewer.svelte";
    import Editor from "./modules/Editor.svelte";
    import Browser from "./modules/Browser.svelte";

    $: {
        if ('error' in $c) {
            document.title = 'How did I get here';
        } else {
            document.title = `${$c.name} - DnDCent`;
        }
    }
</script>

{#if 'error' in $c}
    <div class="uk-padding">
        <h1>D&D machine broke.</h1>
        <p>Understandable, have a nice day.</p>
        <p class="uk-text-danger">{$c.error}</p>
    </div>
{:else}
    <div class="uk-flex">
        <Sidebar {c} {view}/>
        <div class="uk-padding uk-width-expand" id="sheet">
            {#if $view === 'view'}
                <Viewer {c}/>
            {:else if $view === 'edit'}
                <Editor {c}/>
            {:else if $view === 'browse'}
                <Browser/>
            {/if}
        </div>
    </div>
{/if}