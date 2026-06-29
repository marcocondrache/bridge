-- Full-text search over history. External-content FTS5 mirrors `requests`,
-- kept in sync by triggers (the SQLite-recommended pattern).
CREATE VIRTUAL TABLE requests_fts USING fts5(
    url,
    request_body,
    response_body,
    content='requests',
    content_rowid='id'
);

INSERT INTO requests_fts(rowid, url, request_body, response_body)
    SELECT id, url, request_body, response_body FROM requests;

CREATE TRIGGER requests_ai AFTER INSERT ON requests BEGIN
    INSERT INTO requests_fts(rowid, url, request_body, response_body)
    VALUES (new.id, new.url, new.request_body, new.response_body);
END;

CREATE TRIGGER requests_ad AFTER DELETE ON requests BEGIN
    INSERT INTO requests_fts(requests_fts, rowid, url, request_body, response_body)
    VALUES ('delete', old.id, old.url, old.request_body, old.response_body);
END;

CREATE TRIGGER requests_au AFTER UPDATE ON requests BEGIN
    INSERT INTO requests_fts(requests_fts, rowid, url, request_body, response_body)
    VALUES ('delete', old.id, old.url, old.request_body, old.response_body);
    INSERT INTO requests_fts(rowid, url, request_body, response_body)
    VALUES (new.id, new.url, new.request_body, new.response_body);
END;
