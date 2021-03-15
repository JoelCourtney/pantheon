<script lang="ts">
    import { signedInt } from '../helpers.ts';

    export let c;
</script>

<div class="sheet-box uk-width-auto">
    <h1 class="box-title">Skillz</h1>
    <div uk-grid class="uk-grid-small">
        <div class="uk-width-auto uk-text-right">
            <ul class="uk-list uk-list-collapse">
                <li class="proficiency-{c.acrobatics_proficiency.toLowerCase()}">Acrobatics</li>
                <li class="proficiency-{c.animal_handling_proficiency.toLowerCase()}">Animal Handling</li>
                <li class="proficiency-{c.arcana_proficiency.toLowerCase()}">Arcana</li>
                <li class="proficiency-{c.athletics_proficiency.toLowerCase()}">Athletics</li>
                <li class="proficiency-{c.deception_proficiency.toLowerCase()}">Deception</li>
                <li class="proficiency-{c.history_proficiency.toLowerCase()}">History</li>
                <li class="proficiency-{c.insight_proficiency.toLowerCase()}">Insight</li>
                <li class="proficiency-{c.intimidation_proficiency.toLowerCase()}">Intimidation</li>
                <li class="proficiency-{c.investigation_proficiency.toLowerCase()}">Investigation</li>
                <li class="proficiency-{c.medicine_proficiency.toLowerCase()}">Medicine</li>
                <li class="proficiency-{c.nature_proficiency.toLowerCase()}">Nature</li>
                <li class="proficiency-{c.perception_proficiency.toLowerCase()}">Perception</li>
                <li class="proficiency-{c.performance_proficiency.toLowerCase()}">Performance</li>
                <li class="proficiency-{c.persuasion_proficiency.toLowerCase()}">Persuasion</li>
                <li class="proficiency-{c.religion_proficiency.toLowerCase()}">Religion</li>
                <li class="proficiency-{c.sleight_of_hand_proficiency.toLowerCase()}">Sleight of Hand</li>
                <li class="proficiency-{c.stealth_proficiency.toLowerCase()}">Stealth</li>
                <li class="proficiency-{c.survival_proficiency.toLowerCase()}">Survival</li>
            </ul>
        </div>
        <div class="uk-width-auto uk-text-left">
            <ul class="uk-list uk-list-collapse">
                <li>{signedInt(c.acrobatics)}</li>
                <li>{signedInt(c.animal_handling)}</li>
                <li>{signedInt(c.arcana)}</li>
                <li>{signedInt(c.athletics)}</li>
                <li>{signedInt(c.deception)}</li>
                <li>{signedInt(c.history)}</li>
                <li>{signedInt(c.insight)}</li>
                <li>{signedInt(c.intimidation)}</li>
                <li>{signedInt(c.investigation)}</li>
                <li>{signedInt(c.medicine)}</li>
                <li>{signedInt(c.nature)}</li>
                <li>{signedInt(c.perception)}</li>
                <li>{signedInt(c.performance)}</li>
                <li>{signedInt(c.persuasion)}</li>
                <li>{signedInt(c.religion)}</li>
                <li>{signedInt(c.sleight_of_hand)}</li>
                <li>{signedInt(c.stealth)}</li>
                <li>{signedInt(c.survival)}</li>
            </ul>
        </div>
    </div>
</div>