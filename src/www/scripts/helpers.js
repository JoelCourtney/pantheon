let activeView = 'combat';

function setView(view) {
    let className = "view-" + view;
    document.getElementById("menu-" + activeView).classList.remove("uk-active");
    activeView = view;
    document.getElementById("menu-" + activeView).classList.add("uk-active");
    for (let child of document.getElementById("sheet").children) {
        if (child.classList.contains(className)) {
            child.classList.remove("view-hidden");
            child.classList.add("view-visible");
        } else {
            child.classList.remove("view-visible");
            child.classList.add("view-hidden");
        }
    }
}

function getCharacter() {
    const xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 200) {
            callDisplayCallbacks(JSON.parse(this.responseText));
        }
    };
    xhttp.open("POST", "/", true);
    xhttp.setRequestHeader("Content-type", "application/json");
    xhttp.send();
}

function editCharacter(data) {
    const xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState === 4 && this.status === 200) {
            callDisplayCallbacks(JSON.parse(this.responseText));
        }
    };
    xhttp.open("POST", "/edit", true);
    xhttp.setRequestHeader("Content-type", "application/json");
    xhttp.send(JSON.stringify(data));
}

function pasteFields(fields, character, signed=false) {
    if (!signed) {
        fields.forEach(function (field) { document.getElementById('field-' + field).innerHTML = character[field] });
    } else {
        fields.forEach(function (field) { document.getElementById('field-' + field).innerHTML = signedInt(character[field]) });
    }
}

function setField(field, value) {
    document.getElementById("field-"+field).innerHTML = value;
}

function signedInt(int) {
    return (int>=0?"+":"")+int;
}
