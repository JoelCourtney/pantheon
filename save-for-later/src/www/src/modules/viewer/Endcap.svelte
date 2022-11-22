<script lang="ts">
    export let id: number;

    import { onDestroy } from 'svelte';

    function pack() {
        let parent = document.getElementById('endcap-' + id);
        parent.setAttribute('style', 'width: unset; overflow: hidden; float: unset;');
        let width = Math.floor(parent.getBoundingClientRect().width);
        parent.setAttribute('style', 'width: ' + width + 'px; overflow: inherit; float: right');
    }

    setTimeout(pack, 25);
    setTimeout(pack, 200);
    window.addEventListener('resize', pack);
    window.addEventListener('repack', pack);

    onDestroy(() => {
        window.removeEventListener('resize', pack)
        window.removeEventListener('repack', pack)
    });
</script>

<div class="sheet-endcap" id="endcap-{id}">
    <slot/>
</div>
