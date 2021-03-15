import {Writable, writable} from 'svelte/store';

export let character: Writable<Promise<any>> = writable(getCharacter());

async function getCharacter() {
    let c = sendRequest(false, null);
    console.log(c);
    return c;
}

export async function updateCharacter() {
    character.set(getCharacter());
}

export async function editCharacter(request: any) {
    character.set(sendRequest(true, request));
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
        return e;
    }
}

export const view: Writable<string> = writable('builder');