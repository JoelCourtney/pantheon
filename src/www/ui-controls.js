function changeTab(name) {
    $("#combat-body").hide();
    $("#general-body").hide();
    $("#roleplay-body").hide();
    $("#" + name + "-body").show();
}