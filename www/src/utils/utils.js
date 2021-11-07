export function logspace(start, end, total) {
    const div = end / start;
    let step = Math.log(div) / total;
    step = Math.exp(step);
    let val = start;
    return new Array(total).fill(0).map(
        (_, idx) => {
            if (idx !== 0) val = val * step
            return val
        }
    )
}