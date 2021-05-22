<script lang="ts">
    import { getDescription, getRegistry } from "../state";
    import { render, browserHistory } from "../helpers";

    let location = browserHistory.location();

    let registry: Array<any> = [];
    getRegistry().then((value) => {
        value.sort((a, b) => a.name > b.name)
        registry = value;
    });

    let search: string = '';
</script>

<div class="uk-grid-large" uk-grid>
    <div class="uk-width-1-4 uk-text-right">
        <div class="uk-search uk-search-default uk-width-1-1">
            <span uk-search-icon></span>
            <input class="uk-search-input" type="search" placeholder="" bind:value={search}>
        </div>
        <div class="uk-margin-top">
            <ul class="uk-list uk-list-large">
                {#each registry as reg, i}
                    {#if
                        reg.collection.toLowerCase().includes(search.toLowerCase())
                        || reg.source.toLowerCase().includes(search.toLowerCase())
                        || reg.kind.toLowerCase().includes(search.toLowerCase())
                        || reg.name.toLowerCase().includes(search.toLowerCase())
                    }
                        <li><a on:click={
                            () => {
                                location = reg;
                                browserHistory.push(reg);
                            }
                        }>{reg.name}</a><br/><span class="uk-text-small">{reg.kind}<br/>{reg.collection}/{reg.source}</span></li>
                    {/if}
                {/each}
            </ul>
        </div>
    </div>
    {#if location !== null}
        <div class="uk-width-2-3">
            <p>
                <span href="" class="uk-icon-button" uk-icon="icon: arrow-left; ratio: 2" on:click={
                    () => {location = browserHistory.back()}
                }></span>
                <span href="" class="uk-icon-button" uk-icon="icon: arrow-right; ratio: 2" on:click={
                    () => {location = browserHistory.forward()}
                }></span>
                &nbsp;&nbsp;&nbsp;&nbsp;{location.collection} / {location.source} / {location.kind}
            </p>
            {#await getDescription(location)}
            {:then description}
                {@html render(description)}
            {/await}
        </div>
    {/if}
</div>