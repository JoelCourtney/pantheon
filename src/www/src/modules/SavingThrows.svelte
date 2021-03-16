<script lang="ts">
    import { signedInt, render } from '../helpers.ts';

    export let c;
</script>

<div class="sheet-box uk-width-auto">
    <h1 class="box-title">Saving Throws</h1>
    <div class="uk-width-shrink">
        <div uk-grid class="uk-grid-small uk-flex-center">
            <div class="uk-text-right">
                <ul class="uk-list uk-list-collapse">
                    <li class="proficiency-{$c.save_proficiencies.strength.toLowerCase()}">Strength</li>
                    <li class="proficiency-{$c.save_proficiencies.dexterity.toLowerCase()}">Dexterity</li>
                    <li class="proficiency-{$c.save_proficiencies.constitution.toLowerCase()}">Constitution</li>
                    <li class="proficiency-{$c.save_proficiencies.intelligence.toLowerCase()}">Intelligence</li>
                    <li class="proficiency-{$c.save_proficiencies.wisdom.toLowerCase()}">Wisdom</li>
                    <li class="proficiency-{$c.save_proficiencies.charisma.toLowerCase()}">Charisma</li>
                </ul>
            </div>
            <div class="uk-text-left">
                <ul class="uk-list uk-list-collapse">
                    <li>{signedInt($c.saves.strength)}</li>
                    <li>{signedInt($c.saves.dexterity)}</li>
                    <li>{signedInt($c.saves.constitution)}</li>
                    <li>{signedInt($c.saves.intelligence)}</li>
                    <li>{signedInt($c.saves.wisdom)}</li>
                    <li>{signedInt($c.saves.charisma)}</li>
                </ul>
            </div>
        </div>
    </div>
    {#if $c.saving_throw_notes.length}
        <p class="uk-margin-remove-bottom">{@html render($c.saving_throw_notes.join('  \n'))}</p>
    {/if}
</div>