<script lang="ts">
    export let c;

    import Health from './viewer/Health.svelte';
    import ArmorClass from './viewer/ArmorClass.svelte';
    import ProficiencyBonus from './viewer/ProficiencyBonus.svelte';
    import Initiative from './viewer/Initiative.svelte';
    import Speed from './viewer/Speed.svelte';
    import Inspiration from './viewer/Inspiration.svelte';
    import Abilities from './viewer/Abilities.svelte';
    import Hands from './viewer/Hands.svelte';
    import Armor from './viewer/Armor.svelte';
    import Inventory from './viewer/Inventory.svelte';
    import Money from './viewer/Money.svelte';
    import Moves from './viewer/Moves.svelte';
    import Skills from './viewer/Skills.svelte';
    import SavingThrows from "./viewer/SavingThrows.svelte";
    import Proficiencies from "./viewer/Proficiencies.svelte";
    import Passives from "./viewer/Passives.svelte";
    import Statuses from "./viewer/Statuses.svelte";
    import Endcap from "./viewer/Endcap.svelte";
    import ClearLeft from "./viewer/ClearLeft.svelte";
</script>

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