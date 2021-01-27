registerDisplayCallback(function (character) {
    pasteFields(['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma']);
    pasteFields(
        ['strength_modifier', 'dexterity_modifier', 'constitution_modifier', 'intelligence_modifier', 'wisdom_modifier', 'charisma_modifier'],
        true
    );
});