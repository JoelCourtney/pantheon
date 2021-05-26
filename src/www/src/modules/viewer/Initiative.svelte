<script lang="ts">
    export let c;

    import {signedInt} from "../../helpers";
</script>

<div class="sheet-box">
    <h1 class="box-title">Initiative</h1>
    <p class="sheet-box-number">{signedInt($c.initiative)}</p>
</div>