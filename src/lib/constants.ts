export const METHOD_COLORS: Record<string, string> = {
  GET: "text-green-500",
  POST: "text-blue-500",
  PUT: "text-amber-500",
  PATCH: "text-purple-500",
  DELETE: "text-red-500",
  HEAD: "text-gray-500",
  OPTIONS: "text-gray-500",
};

export const METHOD_BG_COLORS: Record<string, string> = {
  GET: "bg-green-500/10",
  POST: "bg-blue-500/10",
  PUT: "bg-amber-500/10",
  PATCH: "bg-purple-500/10",
  DELETE: "bg-red-500/10",
  HEAD: "bg-gray-500/10",
  OPTIONS: "bg-gray-500/10",
};

export const ALL_METHODS = [
  "GET",
  "POST",
  "PUT",
  "PATCH",
  "DELETE",
  "HEAD",
  "OPTIONS",
] as const;

export type HttpMethod = (typeof ALL_METHODS)[number];

export interface HistoryItem {
  method: HttpMethod;
  url: string;
  status: number;
  ms: number;
  ts: string;
}

export const HISTORY: HistoryItem[] = [
  { method: "GET", url: "https://api.github.com/users/octocat", status: 200, ms: 234, ts: "12:41" },
  { method: "POST", url: "https://api.stripe.com/v1/customers", status: 201, ms: 445, ts: "12:38" },
  { method: "GET", url: "https://jsonplaceholder.typicode.com/posts/1", status: 200, ms: 89, ts: "12:35" },
  { method: "DELETE", url: "https://api.example.com/users/42", status: 204, ms: 123, ts: "12:31" },
  { method: "GET", url: "https://api.openai.com/v1/models", status: 401, ms: 67, ts: "12:28" },
  { method: "POST", url: "https://httpbin.org/post", status: 200, ms: 312, ts: "12:25" },
  { method: "PATCH", url: "https://api.example.com/users/42", status: 200, ms: 178, ts: "12:20" },
  { method: "GET", url: "https://api.coinbase.com/v2/prices/BTC-USD/spot", status: 200, ms: 156, ts: "12:15" },
  { method: "POST", url: "https://api.sendgrid.com/v3/mail/send", status: 202, ms: 521, ts: "11:58" },
  { method: "GET", url: "https://api.github.com/repos/torvalds/linux", status: 200, ms: 287, ts: "11:45" },
  { method: "PUT", url: "https://api.example.com/products/99", status: 200, ms: 195, ts: "11:30" },
  { method: "GET", url: "https://api.nasa.gov/planetary/apod", status: 200, ms: 344, ts: "11:12" },
];

export const SAMPLE_RESP_BODY = JSON.stringify(
  {
    login: "octocat",
    id: 583231,
    node_id: "MDQ6VXNlcjU4MzIzMQ==",
    type: "User",
    site_admin: false,
    name: "The Octocat",
    company: "@github",
    blog: "https://github.blog",
    location: "San Francisco, CA",
    email: null,
    public_repos: 8,
    public_gists: 8,
    followers: 9000,
    following: 9,
    created_at: "2011-01-25T18:44:36Z",
    updated_at: "2024-01-15T09:12:00Z",
  },
  null,
  2,
);

export const SAMPLE_RESP_HEADERS: Record<string, string> = {
  "content-type": "application/json; charset=utf-8",
  "x-ratelimit-limit": "60",
  "x-ratelimit-remaining": "58",
  "x-ratelimit-reset": "1713628800",
  "x-github-request-id": "A1B2:C3D4:E5F6:G7H8",
  "cache-control": "public, max-age=60, s-maxage=60",
  etag: '"abc123def456"',
};

export interface KVRow {
  key: string;
  value: string;
  on: boolean;
}

export const REQ_HEADERS: KVRow[] = [
  { key: "Accept", value: "application/vnd.github+json", on: true },
  { key: "Authorization", value: "Bearer ghp_xxxxxxxxxxxx", on: true },
  { key: "X-GitHub-Api-Version", value: "2022-11-28", on: true },
];

export const REQ_PARAMS: KVRow[] = [
  { key: "per_page", value: "30", on: false },
  { key: "page", value: "1", on: false },
];

export function statusColor(status: number): string {
  if (status >= 500) return "text-red-500";
  if (status >= 400) return "text-amber-500";
  if (status >= 300) return "text-blue-500";
  if (status >= 200) return "text-green-500";
  return "text-muted-foreground";
}
