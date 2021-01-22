registerDisplayCallback(function (character) {
    setField("platinum", character.money[0]);
    setField("gold", character.money[1]);
    setField("electrum", character.money[2]);
    setField("silver", character.money[3]);
    setField("copper", character.money[4]);
});