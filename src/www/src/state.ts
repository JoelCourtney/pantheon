import {Writable, writable} from 'svelte/store';
import type {Registration} from './helpers';

export let c: Writable<any> = writable({error: "haven't fetched character yet."});
updateCharacter();

async function getCharacter() {
    return sendRequest('', null);
}

export function updateCharacter() {
    getCharacter().then((value) => c.set(value));
}

export async function editCharacter(request: any) {
    console.log(request);
    sendRequest('/edit', request).then((value) => {
        c.set(value)
        window.dispatchEvent(new CustomEvent('repack'));
    });
}

export async function getRegistry() {
    return sendRequest('/registry', null);
}

export async function getDescription(reg: Registration) {
    return sendRequest('/description', reg);
}

async function sendRequest(path: string, data: any) {
    const location = window.location.hostname;
    const port = window.location.port;
    let settings: any = {
        method: 'POST',
        headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
        }
    };
    if (data !== null) {
        settings.body = JSON.stringify(data);
    }
    try {
        const url = `http://${location}:${port}${path}`;
        const fetchResponse = await fetch(url, settings);
        return await fetchResponse.json();
    } catch (e) {
        return {error: e.toString()};
    }
}

export const view: Writable<string> = writable('browse');

