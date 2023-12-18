export function rgbToHex(r: number, g: number, b: number): string {
    // 各成分を16進数に変換
    const hexR = r.toString(16).padStart(2, '0');
    const hexG = g.toString(16).padStart(2, '0');
    const hexB = b.toString(16).padStart(2, '0');

    // Hex形式に組み合わせて返す
    return `#${hexR}${hexG}${hexB}`;
}

