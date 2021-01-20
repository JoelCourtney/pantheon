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

function pasteFields(fields, character) {
    fields.forEach(function (field) { document.getElementById('field-' + field).innerHTML = character[field] });
}