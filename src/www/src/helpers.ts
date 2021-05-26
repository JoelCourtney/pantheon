export function signedInt(i: number): string {
    return `${i>=0?'+':''}${i}`;
}

// @ts-ignore
import "/scripts/remarkable.min.js";
// @ts-ignore
const md = new Remarkable();
export function render(text: string): string {
    let result: string = md.render(text);
    result = result.replace('<table>', '<table class="uk-table uk-table-small uk-divider">');
    return result;
}

export class Registration {
    collection: string;
    source: string;
    kind: string;
    name: string;

    constructor(c: string, s: string, k: string, n: string) {
        this.collection = c;
        this.source = s;
        this.kind = k;
        this.name = n;
    }
}
class BrowserHistory {
    current: number = -1;
    history: Array<Registration>;

    constructor() {
        this.history = [];
    }

    back(): Registration | null {
        if (this.canGoBack()) {
            this.current--;
        }
        return this.location();
    }
    canGoBack(): boolean {
        return this.current > 0;
    }
    forward(): Registration | null {
        if (this.canGoForward()) {
            this.current++;
        }
        return this.location();
    }
    canGoForward(): boolean {
        return this.current < this.history.length - 1;
    }
    location(): Registration | null {
        if (this.current >= 0)
            return this.history[this.current];
        else
            return null;
    }
    push(newLocation: Registration) {
        this.history.splice(this.current + 1);
        this.history.push(newLocation);
        this.current++;
    }
}
export const browserHistory: BrowserHistory = new BrowserHistory();