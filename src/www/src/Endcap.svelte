<script lang="ts">
    export let id: number;

    function pack() {
        let parent = document.getElementById('endcap-' + id);
        parent.setAttribute('style', 'width: unset; overflow: hidden; float: unset;');
        let width = Math.floor(parent.getBoundingClientRect().width);
        parent.setAttribute('style', 'width: ' + width + 'px; overflow: inherit; float: right');
    }

    // pack();
    setTimeout(pack, 25);
    setTimeout(pack, 200);
    window.addEventListener('resize', pack);
    window.addEventListener('repack', pack);
</script>

<div class="sheet-endcap" id="endcap-{id}">
    <slot/>
</div>