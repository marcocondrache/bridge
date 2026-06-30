import type { KeyValuePair } from "@/components/key-value-editor";

export interface HttpResponse {
  body: string;
  elapsed_ms: number;
  headers: [string, string][];
  status: number;
  status_text: string;
}

export const METHODS = [
  "GET",
  "POST",
  "PUT",
  "PATCH",
  "DELETE",
  "HEAD",
  "OPTIONS",
];

export function withParams(url: string, params: KeyValuePair[]): string {
  const qs = new URLSearchParams(
    params.filter((p) => p.key.trim()).map((p) => [p.key, p.value])
  ).toString();
  if (!qs) {
    return url;
  }
  return url + (url.includes("?") ? "&" : "?") + qs;
}

export function formatBytes(bytes: number): string {
  if (bytes < 1024) {
    return `${bytes} B`;
  }
  if (bytes < 1024 * 1024) {
    return `${(bytes / 1024).toFixed(1)} KB`;
  }
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

export function contentType(headers: [string, string][]): string {
  const ct = headers.find(([k]) => k.toLowerCase() === "content-type")?.[1];
  return ct?.split(";")[0]?.trim() ?? "";
}

export function parseCookies(headers: [string, string][]): [string, string][] {
  return headers
    .filter(([k]) => k.toLowerCase() === "set-cookie")
    .map(([, v]) => {
      const [pair, ...attrs] = v.split(";");
      const eq = pair.indexOf("=");
      const name = eq === -1 ? pair.trim() : pair.slice(0, eq).trim();
      const value = eq === -1 ? "" : pair.slice(eq + 1).trim();
      const meta = attrs.map((a) => a.trim()).join("; ");
      return [name, meta ? `${value}  ·  ${meta}` : value];
    });
}
