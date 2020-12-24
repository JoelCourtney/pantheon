/*
Overall process:
1. Load the modules given by 'module' tags in index.html. Each module consists of a mod.html file and a list of scripts. Currently only 'display.js' is loaded, but more scripts will be used later. Each display script must register a displayCallback function, which takes a character object and puts the relevant data into mod.html.
2. Wait until all mod.html's are loaded, and all displayCallbacks are registered.
3. call processContent() which does:
4. call liftZIndices(), which allows dropdowns to exceed the boundaries of their sheet boxes. Otherwise they get truncated.
5. call updateCharacter(). This queries the server for the character object. The readyStateChange callback will call callDisplayCallbacks() when it returns successfully.
6. Wait for events to be triggered by modules. The modules will send requests to the server on their own when the user interacts with the UI. The response will be the updatedCharacter, which will be displayed again as before.
*/

function load() {
    loadModules();
};

/*
Shamelessly stolen from w3schools
https://www.w3schools.com/howto/howto_html_include.asp

modified to query all modules at time of calling, so the requests are parallel.
*/
var modulesRequested = 0;
var modulesReceived = 0;
function loadModules() {
    /*loop through a collection of all HTML elements:*/
    let z = document.getElementsByTagName("div");
    for (let i = 0; i < z.length; i++) {
        let elmnt = z[i];
        /*search for elements with a certain atrribute:*/
        if (elmnt.hasAttribute("module")) {
            modulesRequested++;
            let mod = "modules/" + elmnt.getAttribute("module") + "/";
            let file = mod + "mod.html";
            elmnt.removeAttribute("module");
            /*make an HTTP request using the attribute value as the file name:*/
            let xhttp = new XMLHttpRequest();
            xhttp.onreadystatechange = function() {
                if (this.readyState == 4) {
                    if (this.status == 200) {
                        elmnt.innerHTML = this.responseText;

                    } else if (this.status == 404) {
                        elmnt.innerHTML = "Page not found.";
                    }
                    /*remove the attribute, and call this function once more:*/
                    modulesReceived++;
                    if (areModulesLoaded()) {
                        processContent();
                    }
                }
            }
            xhttp.open("GET", file, true);
            xhttp.send();
            let displayScript = document.createElement('script');
            displayScript.src = mod + "display.js";
            document.head.appendChild(displayScript);
        }
    }
};

function areModulesLoaded() {
    return modulesRequested == modulesReceived && modulesRequested == displayCallbacks.length;
}

var displayCallbacks = [];
function registerDisplayCallback(func) {
    displayCallbacks.push(func);
    if (areModulesLoaded()) {
        processContent();
    }
}

function callDisplayCallbacks(character) {
    displayCallbacks.forEach(func => func(character));
}

function processContent() {
    liftZIndices();
    // somehow wait until all scripts are loaded.
    updateCharacter();
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

function updateCharacter() {
    // TODO("this, later")
}

