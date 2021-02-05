import {Writable, writable} from 'svelte/store';

export let character: Writable<Promise<any>> = writable(getCharacter());

async function getCharacter() {
    const location = window.location.hostname;
    const port = window.location.port;
    const settings = {
        method: 'POST',
        headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
        }
    };
    try {
        const fetchResponse = await fetch(`http://${location}:${port}/`, settings);
        return await fetchResponse.json();
    } catch (e) {
        return e;
    }
}

export const view: Writable<string> = writable('combat');