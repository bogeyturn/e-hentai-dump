export function displayDate(date: {secs: number}) {
    const d = new Date(date.secs * 1000);
    const year = d.getFullYear();
    const month = String(d.getMonth() + 1).padStart(2, '0'); // Months are 0-based
    const day = String(d.getDate()).padStart(2, '0');

    const hours = String(d.getHours()).padStart(2, '0');
    const minutes = String(d.getMinutes()).padStart(2, '0');

    return `${year}-${month}-${day} ${hours}:${minutes}`;
}
export function displayTime(date: {secs: number}) {
    return new Date(date.secs * 1000).toLocaleTimeString("en-GB", {
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
    });
}