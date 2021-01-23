registerDisplayCallback(function (character) {
    pasteFields(['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma'], character);
    pasteFields(
        ['strength_modifier', 'dexterity_modifier', 'constitution_modifier', 'intelligence_modifier', 'wisdom_modifier', 'charisma_modifier'],
        character, true
    );
});