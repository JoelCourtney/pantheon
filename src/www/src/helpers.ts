export function signedInt(i: number): string {
    return `${i>=0?'+':'-'}${i}`;
}