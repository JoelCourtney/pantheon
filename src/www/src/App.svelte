<script lang='ts'>
    import './main.css';

    import {character, view} from './state.ts';

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
    import SavingThrows from "./modules/SavingThrows.svelte";
    import Proficiencies from "./modules/Proficiencies.svelte";
    import Passives from "./modules/Passives.svelte";

    $: $character.then((c) => document.title = `${c.name} - DnDCent`);
</script>

{#await $character}
    <p>Fetching character.</p>
{:then c}
    <div class="uk-flex" id="all-content">
        <Sidebar {c} {view}/>
        <div class="uk-padding uk-width-expand" id="sheet">
            {#if $view === 'everything'}
                <Health {c}/>
                <ArmorClass {c}/>
                <ProficiencyBonus {c}/>
                <Initiative {c}/>
                <Speed {c}/>
                <Inspiration {c}/>
                <Abilities {c}/>
                <Hands {c}/>
                <Armor {c}/>
                <Inventory {c}/>
                <Money {c}/>
                <Moves {c}/>
                <Passives {c}/>
                <Proficiencies {c}/>
                <Skills {c}/>
                <SavingThrows {c}/>
            {:else if $view === 'combat'}
                <Health {c}/>
            {:else if $view === 'roleplay'}
                <Health {c}/>
            {:else if $view === 'equipment'}
                <Health {c}/>
            {:else if $view === 'skills'}
                <Health {c}/>
            {:else if $view === 'description'}
                <Health {c}/>
            {:else if $view === 'notes'}
                <Health {c}/>
            {/if}
        </div>
    </div>
{/await}