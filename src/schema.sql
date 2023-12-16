CREATE TABLE
  IF NOT EXISTS team (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    role VARCHAR(50) NOT NULL,
    units VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    avatar VARCHAR(100) NOT NULL,
    github VARCHAR(100),
    facebook VARCHAR(100),
    linkedin VARCHAR(100),
    twitter VARCHAR(100),
    envelope VARCHAR(100)
  );
