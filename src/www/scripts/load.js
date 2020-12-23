function load() {
    loadModules();
};

/*
Stolen from w3schools
https://www.w3schools.com/howto/howto_html_include.asp

modified to query all includes at time of calling, so the requests are parallel.
*/
var includesRequested = 0;
var includesReceived = 0;
function loadModules() {
    /*loop through a collection of all HTML elements:*/
    let z = document.getElementsByTagName("div");
    for (let i = 0; i < z.length; i++) {
        let elmnt = z[i];
        /*search for elements with a certain atrribute:*/
        if (elmnt.hasAttribute("module")) {
            let mod = "modules/" + elmnt.getAttribute("module") + "/";
            let file = mod + "mod.html";
            elmnt.removeAttribute("module");
            /*make an HTTP request using the attribute value as the file name:*/
            let xhttp = new XMLHttpRequest();
            xhttp.onreadystatechange = function() {
                if (this.readyState == 4) {
                    if (this.status == 200) {
                        elmnt.innerHTML = this.responseText;
                        var displayScript = document.createElement('script');
//                        displayScript.onload = function () {
                            //do stuff with the script
//                        };
                        displayScript.src = mod + "display.js";
                        document.head.appendChild(displayScript);
                    } else if (this.status == 404) {
                        elmnt.innerHTML = "Page not found.";
                    }
                    /*remove the attribute, and call this function once more:*/
                    includesReceived++;
                    if (includesReceived == includesRequested) {
                        includesRequested = 0;
                        includesReceived = 0;
                        loadModules();
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
    // somehow wait until all scripts are loaded.
    getCharacter();
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

function getCharacter() {
    // TODO("this, later")
}

var displayCallbacks = [];
function registerDisplayCallback(func) {
    displayCallbacks.push(func);
}

function distributeToDisplayCallbacks(character) {
    displayCallbacks.forEach(func => func(character));
}