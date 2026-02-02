import { readFile } from "fs/promises";
import { join } from "path";
import { defineEventHandler, getRequestURL } from "h3";

export default defineEventHandler(async (event) => {
  const url = getRequestURL(event);
  if (url.pathname !== "/opensearch.xml") return;

  const filePath = join(
    process.cwd(),
    "public",
    "g",
    "opensearchdescription.xml",
  );
  let xml = await readFile(filePath, "utf-8");

  const origin = `${url.protocol}//${url.host}`;
  xml = xml.replaceAll("{ORIGIN}", origin);

  event.node.res.setHeader(
    "Content-Type",
    "application/opensearchdescription+xml",
  );
  event.node.res.end(xml);
});
