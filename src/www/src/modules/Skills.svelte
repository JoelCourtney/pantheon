<script lang="ts">
    import { signedInt } from '../helpers.ts';

    export let c;
</script>

<div class="sheet-box uk-width-auto">
    <h1 class="box-title">Skillz</h1>
    <div uk-grid class="uk-grid-small">
        <div class="uk-width-auto uk-text-right">
            <ul class="uk-list uk-list-collapse">
                <li class="proficiency-{c.skill_proficiencies.acrobatics.toLowerCase()}">Acrobatics</li>
                <li class="proficiency-{c.skill_proficiencies.animal_handling.toLowerCase()}">Animal Handling</li>
                <li class="proficiency-{c.skill_proficiencies.arcana.toLowerCase()}">Arcana</li>
                <li class="proficiency-{c.skill_proficiencies.athletics.toLowerCase()}">Athletics</li>
                <li class="proficiency-{c.skill_proficiencies.deception.toLowerCase()}">Deception</li>
                <li class="proficiency-{c.skill_proficiencies.history.toLowerCase()}">History</li>
                <li class="proficiency-{c.skill_proficiencies.insight.toLowerCase()}">Insight</li>
                <li class="proficiency-{c.skill_proficiencies.intimidation.toLowerCase()}">Intimidation</li>
                <li class="proficiency-{c.skill_proficiencies.investigation.toLowerCase()}">Investigation</li>
                <li class="proficiency-{c.skill_proficiencies.medicine.toLowerCase()}">Medicine</li>
                <li class="proficiency-{c.skill_proficiencies.nature.toLowerCase()}">Nature</li>
                <li class="proficiency-{c.skill_proficiencies.perception.toLowerCase()}">Perception</li>
                <li class="proficiency-{c.skill_proficiencies.performance.toLowerCase()}">Performance</li>
                <li class="proficiency-{c.skill_proficiencies.persuasion.toLowerCase()}">Persuasion</li>
                <li class="proficiency-{c.skill_proficiencies.religion.toLowerCase()}">Religion</li>
                <li class="proficiency-{c.skill_proficiencies.sleight_of_hand.toLowerCase()}">Sleight of Hand</li>
                <li class="proficiency-{c.skill_proficiencies.stealth.toLowerCase()}">Stealth</li>
                <li class="proficiency-{c.skill_proficiencies.survival.toLowerCase()}">Survival</li>
            </ul>
        </div>
        <div class="uk-width-auto uk-text-left">
            <ul class="uk-list uk-list-collapse">
                <li>{signedInt(c.skills.acrobatics)}</li>
                <li>{signedInt(c.skills.animal_handling)}</li>
                <li>{signedInt(c.skills.arcana)}</li>
                <li>{signedInt(c.skills.athletics)}</li>
                <li>{signedInt(c.skills.deception)}</li>
                <li>{signedInt(c.skills.history)}</li>
                <li>{signedInt(c.skills.insight)}</li>
                <li>{signedInt(c.skills.intimidation)}</li>
                <li>{signedInt(c.skills.investigation)}</li>
                <li>{signedInt(c.skills.medicine)}</li>
                <li>{signedInt(c.skills.nature)}</li>
                <li>{signedInt(c.skills.perception)}</li>
                <li>{signedInt(c.skills.performance)}</li>
                <li>{signedInt(c.skills.persuasion)}</li>
                <li>{signedInt(c.skills.religion)}</li>
                <li>{signedInt(c.skills.sleight_of_hand)}</li>
                <li>{signedInt(c.skills.stealth)}</li>
                <li>{signedInt(c.skills.survival)}</li>
            </ul>
        </div>
    </div>
</div>