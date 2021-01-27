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
    xhttp.open("POST", "/", false);
    xhttp.setRequestHeader("Content-type", "application/json");
    xhttp.send();
    let text = xhttp.responseText;
    console.log("size: " + text.length + " bytes");
    return JSON.parse(text);
}

function displayCharacter() {
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

let fieldsToPaste = [];
function pasteFields(fields, signed=false) {
    fields.forEach(function (field) {
        fieldsToPaste.push([field, signed]);
    });
}

function actuallyPasteFields(character) {
    fieldsToPaste.forEach(function (paste) {
        document.getElementById('field-' + paste[0]).innerHTML =
            paste[1]?signedInt(character[paste[0]]):character[paste[0]];
    });
}

function setField(field, value) {
    document.getElementById("field-"+field).innerHTML = value;
}

function signedInt(int) {
    return (int>=0?"+":"")+int;
}
