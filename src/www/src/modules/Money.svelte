<script lang="ts">
    export let c;
</script>

<div class="sheet-box uk-width-auto">
    <h1 class="box-title">Currency</h1>
    <div uk-grid class="uk-grid-small">
        <div class="uk-width-auto uk-text-right">
            <ul class="uk-list uk-list-collapse">
                <li>Platinum</li>
                <li>Gold</li>
                <li>Electrum</li>
                <li>Silver</li>
                <li>Copper</li>
            </ul>
        </div>
        <div class="uk-width-auto uk-text-left">
            <ul class="uk-list uk-list-collapse">
                <li>{c.money.platinum}</li>
                <li>{c.money.gold}</li>
                <li>{c.money.electrum}</li>
                <li>{c.money.silver}</li>
                <li>{c.money.copper}</li>
            </ul>
        </div>
    </div>
</div>