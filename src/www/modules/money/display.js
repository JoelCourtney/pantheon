registerDisplayCallback(function (character) {
    setField("copper", character.money[0]);
    setField("silver", character.money[1]);
    setField("electrum", character.money[2]);
    setField("gold", character.money[3]);
    setField("platinum", character.money[4]);
});
