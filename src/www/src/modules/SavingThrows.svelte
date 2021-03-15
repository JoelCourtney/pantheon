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
                    <li class="proficiency-{c.strength_save_proficiency.toLowerCase()}">Strength</li>
                    <li class="proficiency-{c.dexterity_save_proficiency.toLowerCase()}">Dexterity</li>
                    <li class="proficiency-{c.constitution_save_proficiency.toLowerCase()}">Constitution</li>
                    <li class="proficiency-{c.intelligence_save_proficiency.toLowerCase()}">Intelligence</li>
                    <li class="proficiency-{c.wisdom_save_proficiency.toLowerCase()}">Wisdom</li>
                    <li class="proficiency-{c.charisma_save_proficiency.toLowerCase()}">Charisma</li>
                </ul>
            </div>
            <div class="uk-text-left">
                <ul class="uk-list uk-list-collapse">
                    <li>{signedInt(c.strength_save)}</li>
                    <li>{signedInt(c.dexterity_save)}</li>
                    <li>{signedInt(c.constitution_save)}</li>
                    <li>{signedInt(c.intelligence_save)}</li>
                    <li>{signedInt(c.wisdom_save)}</li>
                    <li>{signedInt(c.charisma_save)}</li>
                </ul>
            </div>
        </div>
    </div>
    {#if c.saving_throw_notes.length}
        <p class="uk-margin-remove-bottom">{@html render(c.saving_throw_notes.join('  \n'))}</p>
    {/if}
</div>