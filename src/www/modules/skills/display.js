registerDisplayCallback(function (character) {
    let skills = [
        'acrobatics',
        'animal_handling',
        'arcana',
        'athletics',
        'deception',
        'history',
        'insight',
        'intimidation',
        'investigation',
        'medicine',
        'nature',
        'perception',
        'performance',
        'persuasion',
        'religion',
        'sleight_of_hand',
        'stealth',
        'survival'
    ];
    pasteFields(skills, true);

    for (skill of skills) {
        let prof = character[skill + "_proficiency"];
        if (prof !== "None") {
            document.getElementById('label-' + skill).classList.add("proficiency-" + prof.toLowerCase());
        }
    }
    // TODO vantage
});