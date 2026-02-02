import { promises as fs } from 'fs';
import { resolve } from 'path';
import { defineEventHandler, getQuery } from 'h3';

export default defineEventHandler(async (event) => {
    const req = event.node.req;
    const url_ = req.url || "";
    if (!url_.startsWith("/save")) return;
    const query = getQuery(event);
    const id = query.id as string;
    const unit = query.unit as string;
    const task = query.task as string;

    if (!id || !unit || !task) {
        return { success: false, message: 'Missing parameters' };
    }

    const folderPath = resolve(process.cwd(), "..", task);
    const filePath = resolve(folderPath, id);

    try {
        await fs.mkdir(folderPath, { recursive: true });

        await fs.writeFile(filePath, unit, 'utf-8');

        return { success: true, message: `Saved ${filePath}` };
    } catch (err) {
        console.error(err);
        return { success: false, message: 'Error saving file' };
    }
});