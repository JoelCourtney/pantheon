import {Writable, writable} from 'svelte/store';

class State {
    name: string = ''
    race_name: string = ''
    class_names: Array<string> = []
}

export const c: Writable<State> = writable(new State());
getCharacter()
    .then((value) => c.set(value));

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