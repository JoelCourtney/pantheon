<script lang="ts">
    export let c;

    let expandedTools = c.tool_proficiencies.map(
        tool_prof => {
            let tool = tool_prof[0];
            let prof = tool_prof[1];
            if (prof === 'Double') {
                return '<span class="proficiency-double">' + tool + '</span>';
            } else {
                return tool;
            }
        }
    );
</script>

<div class="sheet-box uk-width-medium">
    <h1 class="sheet-box-title-small">Proficiencies</h1>
    <dl class="uk-description-list uk-text-left">
        <dt>Armor</dt>
        <dd>{c.armor_proficiencies.join(', ')}</dd>
        <dt>Weapons</dt>
        <dd>{c.weapon_proficiencies.join(', ')}</dd>
        <dt>Tools</dt>
        <dd>{@html expandedTools.join(', ')}</dd>
        <dt>Language</dt>
        <dd>{c.languages.join(', ')}</dd>
    </dl>
</div>