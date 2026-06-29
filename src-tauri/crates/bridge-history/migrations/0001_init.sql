CREATE TABLE requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    method TEXT NOT NULL,
    url TEXT NOT NULL,
    status INTEGER NOT NULL,
    request_headers TEXT NOT NULL,
    request_body TEXT,
    response_headers TEXT NOT NULL,
    response_body TEXT NOT NULL,
    elapsed_ms INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE INDEX idx_requests_created_at ON requests(created_at DESC);
CREATE INDEX idx_requests_url ON requests(url);
CREATE INDEX idx_requests_method ON requests(method);
CREATE INDEX idx_requests_status ON requests(status);
