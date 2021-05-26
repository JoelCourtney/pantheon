export function signedInt(i: number): string {
    return `${i>=0?'+':''}${i}`;
}

// @ts-ignore
import "/scripts/remarkable.min.js";
// @ts-ignore
const md = new Remarkable();
md.core.ruler.enable([
    'abbr'
]);
export function render(text: string, outerTags: boolean = true): string {
    let result: string = md.render(text);
    result = result.replace('<table>', '<table class="uk-table uk-table-small uk-divider">');
    let start_tag = result.search('<abbr title="');
    while (start_tag != -1) {
        let sub = result.substring(start_tag);
        let end_quote = sub.search('">')
        let end_tag = sub.search('</abbr>')
        let title = sub.substring(13, end_quote);
        let content = sub.substring(end_quote + 2, end_tag);
        result = result.substring(0, start_tag) +
            '<u>' + content + '</u><span uk-dropdown="pos: bottom-center" class="uk-width-medium">' + title + '</span>' +
            result.substring(result.search('</abbr>') + 7);
        start_tag = result.search('<abbr title="');
    }
    if (outerTags) {
        return result;
    } else {
        return result.substring(3, result.length - 5);
    }
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