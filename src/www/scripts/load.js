/*
Stolen from w3schools
https://www.w3schools.com/howto/howto_html_include.asp

modified to query all includes at time of calling, so the requests are parallel.
*/

function load() {
    includeHTML();
};

var includesRequested = 0;
var includesReceived = 0;
function includeHTML() {
    /*loop through a collection of all HTML elements:*/
    let z = document.getElementsByTagName("div");
    for (let i = 0; i < z.length; i++) {
        let elmnt = z[i];
        /*search for elements with a certain atrribute:*/
        if (elmnt.hasAttribute("include-html")) {
            let file = "includes/" + elmnt.getAttribute("include-html") + ".html";
            elmnt.removeAttribute("include-html");
            /*make an HTTP request using the attribute value as the file name:*/
            let xhttp = new XMLHttpRequest();
            xhttp.onreadystatechange = function() {
                if (this.readyState == 4) {
                    if (this.status == 200) {elmnt.innerHTML = this.responseText;}
                    if (this.status == 404) {elmnt.innerHTML = "Page not found.";}
                    /*remove the attribute, and call this function once more:*/
                    includesReceived++;
                    if (includesReceived == includesRequested) {
                        includesRequested = 0;
                        includesReceived = 0;
                        includeHTML();
                    }
                }
            }
            xhttp.open("GET", file, true);
            xhttp.send();
            includesRequested++;
        }
    }
    if (includesRequested == 0) {
        contentLoaded()
    }
};

function contentLoaded() {
    liftZIndices();
};

function liftZIndices() {
    var offset = 0;
    let drops = document.getElementsByClassName("uk-dropdown");
    for (var e of drops) {
        let z = window.getComputedStyle(e).zIndex;
        while (e.parentElement.id != "sheet") {
            e = e.parentElement;
        }
        e.style.zIndex = z - offset;
        offset++;
    }
}