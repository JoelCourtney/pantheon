<script lang='ts'>
    import './main.css';

    import { character, view } from './state.ts';

    import Sidebar from './modules/Sidebar.svelte';
    import Health from './modules/Health.svelte';
    import ArmorClass from './modules/ArmorClass.svelte';
    import ProficiencyBonus from './modules/ProficiencyBonus.svelte';
    import Initiative from './modules/Initiative.svelte';
    import Speed from './modules/Speed.svelte';
    import Inspiration from './modules/Inspiration.svelte';
    import Abilities from './modules/Abilities.svelte';
    import Hands from './modules/Hands.svelte';
    import Armor from './modules/Armor.svelte';
    import Inventory from './modules/Inventory.svelte';
    import Money from './modules/Money.svelte';
    import Moves from './modules/Moves.svelte';
    import Skills from './modules/Skills.svelte';

    $: $character.then((c) => document.title = `${c.name} - DnDCent`);
</script>

{#await $character}
    <p>Fetching character.</p>
{:then c}
    <div class="uk-flex" id="all-content">
        <Sidebar {c} {view}/>
        <div class="uk-padding uk-width-expand" id="sheet">
            <Health {c} {view}/>
            <ArmorClass {c} {view}/>
            <ProficiencyBonus {c} {view}/>
            <Initiative {c} {view}/>
            <Speed {c} {view}/>
            <Inspiration {c} {view}/>
            <Abilities {c} {view}/>
            <Hands {c} {view}/>
            <Armor {c} {view}/>
            <Inventory {c} {view}/>
            <Money {c} {view}/>
            <Moves {c} {view}/>
            <Skills {c} {view}/>
        </div>
    </div>
{/await}