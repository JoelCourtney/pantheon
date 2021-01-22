registerDisplayCallback(function (character) {
    setField("initiative", ((character.initiative>=0)?"+":"")+character.initiative);
});