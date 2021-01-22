registerDisplayCallback(function (character) {
    let field = document.getElementById("field-inspiration");
    if (character.inspiration) {
        field.setAttribute("uk-icon", "icon: warning; ratio: 2;");
    } else {
        field.setAttribute("uk-icon", "icon: ban; ratio: 2;");
    }
});