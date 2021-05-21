<script lang="ts">
    import {signedInt} from "../helpers";

    export let c;

    let attackActions = 0;
    let castActions = 0;
    for (let move of $c.moves) {
        if (move.type === 'Attack' && move.time === 'Action') attackActions++;
        if (move.type === 'Cast' && move.time === 'Action') castActions++;
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
        {#if attackActions !== 0}
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
                    {#each $c.moves as move}
                        {#if move.type === 'Attack' && move.time === 'Action'}
                            <tr>
                                <td>{move.name}</td>
                                <td>{signedInt(move.hit)}</td>
                                <td>{move.range.Fixed} ft</td>
                                <td>{move.damage}</td>
                                <td>{move.properties.join(', ')}</td>
                            </tr>
                        {/if}
                    {/each}
                </tbody>
            </table>
        {/if}
        {#if castActions !== 0}
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