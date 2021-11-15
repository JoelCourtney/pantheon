<script lang="ts">
    import Classes from "./editor/Classes.svelte";

    export let c;

    import ElementList from "./ElementList.svelte";

    import Name from "./editor/Name.svelte";
    import Abilities from "./editor/Abilities.svelte";
    import Race from "./editor/Race.svelte";
    import Feats from "./editor/Feats.svelte";
    import Background from "./editor/Background.svelte";
    import Description from "./editor/Description.svelte";


    import {editCharacter} from '../state.ts';
</script>

<div class="uk-grid-row-small uk-grid-column-collapse uk-child-width-2-3 uk-flex-center" uk-grid>
    <Name {c}/>
    <Abilities {c}/>
    <Race {c}/>
    <Classes {c}/>
    <Background {c}/>
    <Feats {c}/>
    <Description {c}/>
</div>