import {Writable, writable} from 'svelte/store';

export let c: Writable<any> = writable({error: "haven't fetched character yet."});
updateCharacter();

async function getCharacter() {
    let c = sendRequest(false, null);
    return c;
}

export function updateCharacter() {
    getCharacter().then((value) => c.set(value));
}

export async function editCharacter(request: any) {
    sendRequest(true, request).then((value) => {
        c.set(value)
        window.dispatchEvent(new CustomEvent('repack'));
    });
}

async function sendRequest(edit: boolean, data: any) {
    const location = window.location.hostname;
    const port = window.location.port;
    let settings: any = {
        method: 'POST',
        headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
        }
    };
    if (edit) {
        settings.body = JSON.stringify(data);
    }
    try {
        const url = edit
            ?`http://${location}:${port}/edit/`
            :`http://${location}:${port}/`;
        const fetchResponse = await fetch(url, settings);
        return await fetchResponse.json();
    } catch (e) {
        return {error: e.toString()};
    }
}

export const view: Writable<string> = writable('builder');