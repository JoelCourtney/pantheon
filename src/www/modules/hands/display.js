registerDisplayCallback(function (character) {
    pasteFields(['left_hand', 'right_hand']);
    let leftHandDropdown = document.getElementById('left-hand-dropdown');
    let rightHandDropdown = document.getElementById('right-hand-dropdown');
    for (let item of character.hold_choices) {
        leftHandDropdown.innerHTML += '<li><a>' + item + '</a></li>';
        rightHandDropdown.innerHTML += '<li><a>' + item + '</a></li>';
    }
});