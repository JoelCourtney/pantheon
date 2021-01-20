function pasteFields(fields, character) {
    fields.forEach(function (field) { document.getElementById('field-' + field).innerHTML = character[field] });
}