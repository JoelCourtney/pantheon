registerDisplayCallback(function (character) {
    pasteFields(['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma'], character);
    setField("strength_modifier", signedInt(character.strength_modifier));
    setField("dexterity_modifier", signedInt(character.dexterity_modifier));
    setField("constitution_modifier", signedInt(character.constitution_modifier));
    setField("intelligence_modifier", signedInt(character.intelligence_modifier));
    setField("wisdom_modifier", signedInt(character.wisdom_modifier));
    setField("charisma_modifier", signedInt(character.charisma_modifier));
});