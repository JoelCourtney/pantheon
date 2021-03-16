<script lang="ts">
    export let c;
</script>

<div class="sheet-box uk-width-auto">
    <div uk-grid>
        <div class="uk-inline">
            <h1 class="box-title">Left Hand</h1>
            <button class="uk-button uk-button-default" type="button" id="field-left_hand">{$c.left_hand}</button>
        </div>
        <div class="uk-inline uk-padding-remove-left">
            <h1 class="box-title">Right Hand</h1>
            <button class="uk-button uk-button-default" type="button" id="field-right_hand">{$c.right_hand}</button>
        </div>
    </div>
</div>
<div uk-dropdown="mode: click; duration: 0; toggle: #field-left_hand;">
    <ul class="uk-nav uk-dropdown-nav uk-text-left uk-nav-default" id="left-hand-dropdown">
        <li><a>Swap</a></li>
        <li><a>Stow</a></li>
        <li><a>Drop</a></li>
        <li class="uk-nav-header">Weapons</li>
        {#each $c.hold_choices as choice}
            <li><a>{choice}</a></li>
        {/each}
    </ul>
</div>
<div uk-dropdown="mode: click; duration: 0; toggle: #field-right_hand;">
    <ul class="uk-nav uk-dropdown-nav uk-text-left uk-nav-default" id="right-hand-dropdown">
        <li><a>Swap</a></li>
        <li><a>Stow</a></li>
        <li><a>Drop</a></li>
        <li class="uk-nav-header">Weapons</li>
        {#each $c.hold_choices as choice}
            <li><a>{choice}</a></li>
        {/each}
    </ul>
</div>