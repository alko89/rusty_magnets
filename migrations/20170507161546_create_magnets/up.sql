CREATE TABLE magnets (
  id INTEGER PRIMARY KEY,
  magnet TEXT NOT NULL,
  seeders INTEGER NOT NULL,
  leechers INTEGER NOT NULL,
  name TEXT NOT NULL,
  website_source VARCHAR NOT NULL,
  url TEXT NOT NULL,
  size VARCHAR NOT NULL,
  inserted_at VARCHAR NOT NULL,
  updated_at VARCHAR NOT NULL
)
