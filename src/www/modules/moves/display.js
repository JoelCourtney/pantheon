registerDisplayCallback(function (character) {
    pasteFields(['attacks_per_action']);
    let attackActions = document.getElementById('table-attack_actions');
    let castActions = document.getElementById('table-cast_actions');
    for (attack of character.attack_moves) {
        let range = attack.range.Fixed;
        let row = '<tr><td>'
            + attack.name
            + '</td><td>'
            + signedInt(attack.hit)
            + '</td><td>'
            + range
            + ' ft</td><td>'
            + attack.damage
            + '</td><td>'
            + attack.properties.join(', ')
            + '</td></tr>';
        if (attack.time === 'Action') {
            attackActions.innerHTML += row;
        }
    }
});