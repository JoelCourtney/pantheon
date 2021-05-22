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
                <li class:uk-active={$view==='view'}><a on:click={() => setView('view')}>View</a></li>
                <li class:uk-active={$view==='edit'}><a on:click={() => setView('edit')}>Edit</a></li>
                <li class:uk-active={$view==='browse'}><a on:click={() => setView('browse')}>Browse</a></li>
            </ul>
        </div>
    </div>
</div>