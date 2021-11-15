<script lang="ts">
    export let c;
</script>

<div class="sheet-box uk-width-auto">
    <h1 class="box-title">Armor</h1>
    <button class="uk-button uk-button-default" type="button" id="armor-button">{$c.armor?$c.armor:'None'}</button>
</div>
<div uk-dropdown="mode: click; duration: 0; toggle: #armor-button;">
    <ul class="uk-nav uk-dropdown-nav uk-text-left uk-nav-default">
        <li><a>Doff</a></li>
        <!--                            <li class="uk-nav-header">Armor</li>-->
        <!--                            <li><a>Studded Leather</a></li>-->
    </ul>
</div>