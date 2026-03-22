CREATE TABLE media_metadata (
  id INTEGER PRIMARY KEY REFERENCES media(id) ON DELETE CASCADE,
  authors TEXT[] NOT NULL,
  publisher TEXT,
  isbn TEXT,
  language TEXT,
  published_at TIMESTAMPTZ
)
