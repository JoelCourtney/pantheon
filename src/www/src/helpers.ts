export function signedInt(i: number): string {
    return `${i>=0?'+':'-'}${i}`;
}

// export function packEndcaps() {
//     for (let cap of document.getElementsByClassName('sheet-endcap')) {
//         cap.classList.remove('uk-float-right');
//         cap.setAttribute('style', 'width: auto;')
//         let width = window.getComputedStyle(cap).width;
//         console.log(width);
//         cap.setAttribute('style', `width: ${width};`);
//         cap.classList.add('uk-float-right');
//     }
// }