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
    import Statuses from "./modules/Statuses.svelte";
    import Endcap from "./Endcap.svelte";
    import ClearLeft from "./ClearLeft.svelte";

    import Builder from "./modules/Builder.svelte";

    $: $character.then((c) => {
        document.title = `${c.name} - DnDCent`;
    });
</script>

{#await $character}
    <p>Fetching character.</p>
{:then c}
    <div class="uk-flex">
        <Sidebar {c} {view}/>
        <div class="uk-padding uk-width-expand" id="sheet">
            {#if $view === 'everything'}
                <Health {c}/>
                <ArmorClass {c}/>
                <ProficiencyBonus {c}/>
                <Initiative {c}/>
                <Speed {c}/>
                <Inspiration {c}/>
                <Hands {c}/>
                <Armor {c}/>
                <ClearLeft>
                    <Skills {c}/>
                </ClearLeft>
                <Abilities {c}/>
                <Endcap id={0}>
                    <Moves {c}/>
                </Endcap>
                <SavingThrows {c}/>
                <Passives {c}/>
                <ClearLeft>
                    <Proficiencies {c}/>
                </ClearLeft>
                <Inventory {c}/>
                <Endcap id={1}>
                    <Statuses {c}/>
                </Endcap>
                <Money {c}/>
            {:else if $view === 'combat'}
                <Health {c}/>
                <ArmorClass {c}/>
                <ProficiencyBonus {c}/>
                <Initiative {c}/>
                <Speed {c}/>
                <Inspiration {c}/>
                <Hands {c}/>
                <Armor {c}/>
                <ClearLeft>
                    <Skills {c}/>
                </ClearLeft>
                <Abilities {c}/>
                <Endcap id={0}>
                    <Moves {c}/>
                </Endcap>
                <SavingThrows {c}/>
                <Passives {c}/>
                <Statuses {c}/>
            {:else if $view === 'roleplay'}
                <Health {c}/>
            {:else if $view === 'equipment'}
                <Health {c}/>
            {:else if $view === 'skills'}
                <Skills {c}/>
                <Abilities {c}/>
                <Endcap id={0}>
                    <Proficiencies {c}/>
                </Endcap>
                <SavingThrows {c}/>
                <Passives {c}/>
            {:else if $view === 'description'}
                <Health {c}/>
            {:else if $view === 'notes'}
                <Health {c}/>
            {:else if $view === 'builder'}
                <Builder {c}/>
            {/if}
        </div>
    </div>
{/await}