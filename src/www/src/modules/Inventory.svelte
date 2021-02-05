<script lang="ts">
    export let c;
    export let view;
</script>

<div class="sheet-box uk-width-xlarge uk-height-max-large uk-overflow-auto">
    <h1 class="sheet-box-title-small">Inventory</h1>
    <table class="uk-table uk-table-small uk-table-divider uk-text-left">
        <caption>Weapons</caption>
        <thead>
        <tr>
            <th class="uk-width-small">Name</th>
            <th class="uk-table-shrink">lbs</th>
            <th class="uk-table-shrink">gp</th>
            <th class="uk-table-expand">Properties</th>
        </tr>
        </thead>
        <tbody>
        <tr>
            <td>Dagger x2</td>
            <td>1</td>
            <td>2</td>
            <td>Simple, Finesse, Light, Thrown (20/60)</td>
        </tr>
        <tr>
            <td>Rapier</td>
            <td>2</td>
            <td>25</td>
            <td>Martial, Finesse</td>
        </tr>
        <tr>
            <td>Shortbow</td>
            <td>2</td>
            <td>25</td>
            <td>Simple, Ammunition, Range (80/320), Two-Handed</td>
        </tr>
        </tbody>
    </table>
    <table class="uk-table uk-table-small uk-table-divider uk-text-left">
        <caption>Equipable</caption>
        <thead>
        <tr>
            <th class="uk-table-shrink">Active</th>
            <th class="uk-width-small">Name</th>
            <th class="uk-table-shrink">lbs</th>
            <th class="uk-table-shrink">gp</th>
            <th class="uk-table-expand">Properties</th>
        </tr>
        </thead>
        <tbody>
        <tr>
            <td><span uk-icon="check"></span></td>
            <td>Cloak of Elvenkind</td>
            <td>-</td>
            <td>-</td>
            <td>Deception, Outerwear</td>
        </tr>
        <tr>
            <td><span uk-icon="close"></span></td>
            <td>Shield</td>
            <td>6</td>
            <td>10</td>
            <td>Warding</td>
        </tr>
        <tr>
            <td><span uk-icon="check"></span></td>
            <td>Studded Leather</td>
            <td>13</td>
            <td>45</td>
            <td>Warding</td>
        </tr>
        </tbody>
    </table>
    <table class="uk-table uk-table-small uk-table-divider uk-text-left">
        <caption>Supplies and Tools</caption>
        <thead>
        <tr>
            <th class="uk-width-small">Name</th>
            <th class="uk-table-shrink">lbs</th>
            <th class="uk-table-shrink">gp</th>
            <th class="uk-table-expand">Properties</th>
        </tr>
        </thead>
        <tbody>
        <tr>
            <td>Arrows x20</td>
            <td>1</td>
            <td>1</td>
            <td>Damage, Combat</td>
        </tr>
        <tr>
            <td>Weed</td>
            <td>--</td>
            <td>--</td>
            <td>Weed</td>
        </tr>
        <tr>
            <td>Backpack</td>
            <td>5</td>
            <td>2</td>
            <td>Container</td>
        </tr>
        <tr>
            <td>Bag of Holding</td>
            <td>15</td>
            <td>--</td>
            <td>Utility, Container, Wondrous</td>
        </tr>
        </tbody>
    </table>
</div>