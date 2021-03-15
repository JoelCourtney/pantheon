<script lang="ts">
    export let c;
</script>

<div class="uk-grid-small uk-child-width-2-3 uk-flex-center" uk-grid>
    <div class="builder-box">
        <h1 class="box-title">Name</h1>
        <input class="uk-input" placeholder="Person McPersonFace"/>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Abilities</h1>
        <div class="uk-grid-small" uk-grid>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="STR">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="DEX">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="CON">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="INT">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="WIS">
            </div>
            <div class="uk-width-1-3">
                <input class="uk-input" type="text" placeholder="CHA">
            </div>
        </div>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Race</h1>
        <select class="uk-select">
            <option>Hooman</option>
            <option>Elv</option>
            <option>Dwarv</option>
        </select>
        <ul uk-tab class="builder-box-nav">
            <li><a>Traits</a></li>
            <li><a>Description</a></li>
        </ul>
        <ul class="uk-list uk-list-bullet uk-margin-remove-top">
            <li>Hello there</li>
            <li>Hello there</li>
        </ul>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Classes</h1>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Background</h1>
    </div>
    <div class="builder-box">
        <h1 class="box-title">Description</h1>
        <textarea class="uk-textarea uk-height-large" placeholder="Insert tragic and edgy backstory here."></textarea>
    </div>
</div>