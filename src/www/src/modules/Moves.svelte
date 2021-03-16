<script lang="ts">
    import {signedInt} from "../helpers";

    export let c;

    let attackActions: Array<Array<string>> = [];
    let castActions: string = '';
    for (let attack of $c.attack_moves) {
        let range = attack.range.Fixed;
        let row = [
            attack.name,
            signedInt(attack.hit),
            `${range} ft`,
            attack.damage,
            attack.properties.join(', ')
        ];
        if (attack.time === 'Action') {
            attackActions.push(row);
        }
    }
</script>

<div class="sheet-box uk-width-2xlarge">
    <ul uk-tab class="sheet-box-nav-small">
        <li><a>Actions</a></li>
        <li><a>Boneless Actions</a></li>
        <li><a>Reactions</a></li>
        <li><a>Other</a></li>
    </ul>
    <div id="move-actions">
        {#if attackActions.length}
            <table class="uk-table uk-table-small uk-table-divider uk-text-left">
                <caption>Attacks ({$c.attacks_per_action} per action)</caption>
                <thead>
                <tr>
                    <th class="uk-width-small">Weapon</th>
                    <th class="uk-table-shrink">+Hit</th>
                    <th class="uk-table-shrink">Range</th>
                    <th class="uk-width-small">Damage</th>
                    <th class="uk-table-expand">Properties</th>
                </tr>
                </thead>
                <tbody>
                {#each attackActions as attack}
                    <tr>
                        {#each attack as cell}
                            <td>{cell}</td>
                        {/each}
                    </tr>
                {/each}
                </tbody>
            </table>
        {/if}
        {#if castActions.length}
            <table class="uk-table uk-table-small uk-table-divider uk-text-left">
                <caption>Casts</caption>
                <thead>
                <tr>
                    <th class="uk-width-small">Spell</th>
                    <th class="uk-table-shrink">+Hit/DC</th>
                    <th class="uk-table-shrink">Range</th>
                    <th class="uk-table-shrink">Level</th>
                    <th class="uk-table-expand">Properties</th>
                </tr>
                </thead>
                <tbody id="table-cast_actions">
                </tbody>
            </table>
        {/if}
    </div>
</div>