<script lang='ts'>
    import './main.css';

    import { c, view } from './state.ts';

    import Sidebar from './modules/Sidebar.svelte';
    import Health from './modules/Health.svelte';

    $: document.title = `${$c.name} - DnDCent`;
</script>

<div class="uk-flex" id="all-content">
    <Sidebar {c} {view}/>
    <div class="uk-padding uk-width-expand" id="sheet">
        <Health {c} {view}/>
<!--        <div class="view-everything view-combat" module="armor-class"></div>-->
<!--        <div class="view-everything view-combat view-roleplay" module="proficiency-bonus"></div>-->
<!--        <div class="view-everything view-combat" module="initiative"></div>-->
<!--        <div class="view-everything view-combat" module="speed"></div>-->
<!--        <div class="view-everything view-combat view-roleplay" module="inspiration"></div>-->
<!--        <div class="view-everything view-combat view-roleplay view-skills" module="abilities"></div>-->
<!--        <div class="view-everything view-combat view-equipment" module="hands" clear="combat"></div>-->
<!--        <div class="view-everything view-combat view-equipment" module="armor"></div>-->
<!--        <div class="view-everything view-equipment" module="inventory"></div>-->
<!--        <div class="view-everything view-roleplay view-equipment" module="money"></div>-->
<!--        <div class="view-everything view-combat" module="moves"></div>-->
<!--        <div class="view-everything view-combat view-roleplay view-skills" module="skills" clear="combat"></div>-->
    </div>
</div>
