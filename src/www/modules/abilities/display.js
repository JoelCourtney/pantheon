registerDisplayCallback(function (character) {
    pasteFields(['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma'], character);
    setField("strength_modifier", ((character.strength_modifier>=0)?"+":"")+character.strength_modifier);
    setField("dexterity_modifier", ((character.dexterity_modifier>=0)?"+":"")+character.dexterity_modifier);
    setField("constitution_modifier", ((character.constitution_modifier>=0)?"+":"")+character.constitution_modifier);
    setField("intelligence_modifier", ((character.intelligence_modifier>=0)?"+":"")+character.intelligence_modifier);
    setField("wisdom_modifier", ((character.wisdom_modifier>=0)?"+":"")+character.wisdom_modifier);
    setField("charisma_modifier", ((character.charisma_modifier>=0)?"+":"")+character.charisma_modifier);
});