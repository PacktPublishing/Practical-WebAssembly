let someGlobalArray = [1, 4, 9, 16, 25];

export function getNumber() {
    return someGlobalArray.length;
}

export function topArray() {
    return someGlobalArray.sort().pop();
}

export lowerCase(str) {
    return str.toLowerCase();
}
