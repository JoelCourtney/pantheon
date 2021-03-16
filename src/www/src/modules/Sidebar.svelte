<script lang="ts">
    export let c;
    export let view;

    let class_names: string;
    $: {
        class_names = $c.class_names[0];
        for (let i = 1; i < $c.class_names.length; i++) {
            class_names += " | " + $c.class_names[i];
        }
    }

    function setView(v: string) {
        $view = v;
    }
</script>

<div class="uk-width-medium uk-padding-remove uk-margin-remove">
    <div class="uk-width-medium" uk-sticky>
        <div class="uk-text-right uk-padding">
            <h1>{$c.name}</h1>
            <p>{$c.race_name}<br/>{class_names}</p>
        </div>
        <div class="uk-text-right uk-padding">
            <ul class="uk-nav uk-nav-primary">
                <li class:uk-active={$view==='everything'} id="menu-everything"><a on:click={() => setView('everything')}>Everything</a></li>
                <li class:uk-active={$view==='combat'} id="menu-combat"><a on:click={() => setView('combat')}>Combat</a></li>
                <li class:uk-active={$view==='roleplay'} id="menu-roleplay"><a on:click={() => setView('roleplay')}>Roleplay</a></li>
                <li class:uk-active={$view==='equipment'} id="menu-equipment"><a on:click={() => setView('equipment')}>Equipment</a></li>
                <li class:uk-active={$view==='skills'} id="menu-skills"><a on:click={() => setView('skills')}>Skills & Proficiencies</a></li>
                <li class:uk-active={$view==='description'} id="menu-description"><a on:click={() => setView('description')}>Description</a></li>
                <li class:uk-active={$view==='notes'} id="menu-notes"><a on:click={() => setView('notes')}>Notes</a></li>
                <li><p></p></li>
                <li class:uk-active={$view==='builder'}><a on:click={() => setView('builder')}>Builder</a></li>
            </ul>
        </div>
    </div>
</div>