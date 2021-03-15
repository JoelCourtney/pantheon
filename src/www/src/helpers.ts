export function signedInt(i: number): string {
    return `${i>=0?'+':'-'}${i}`;
}

// @ts-ignore
import "/scripts/remarkable.min.js";
// @ts-ignore
const md = new Remarkable();
export function render(text: string): string {
    return md.render(text);
}