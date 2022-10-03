CREATE TABLE IF NOT EXISTS users (
    id CHAR(30) PRIMARY KEY,
    lvl INTEGER DEFAULT 0,
    xp INTEGER DEFAULT 0,
    credit INTEGER DEFAULT 0,
    bg TEXT DEFAULT ""
);

CREATE TABLE IF NOT EXISTS quotes (
    quote TEXT NOT NULL,
    quotee CHAR(30) DEFAULT "?",
    quoter CHAR(30) DEFAULT "?",
    qweight REAL DEFAULT 0.5,
    PRIMARY KEY (quote, quotee),
    FOREIGN KEY (quotee) REFERENCES users(id)
      ON DELETE RESTRICT
      ON UPDATE RESTRICT,
    FOREIGN KEY (quoter) REFERENCES users(id)
      ON DELETE RESTRICT
      ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS cards (
    ownr CHAR(30) DEFAULT "?",
    cid INTEGER NOT NULL,
    lvl INTEGER DEFAULT 0,
    tradable INTEGER DEFAULT 1,
    PRIMARY KEY (ownr, cid, lvl),
    FOREIGN KEY (ownr) REFERENCES users(id)
      ON DELETE RESTRICT
      ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS tracks (
    title TEXT NOT NULL,
    artist TEXT DEFAULT "unknown",
    genre TEXT DEFAULT "unknown",
    leng INTEGER DEFAULT 0,
    adder CHAR(30) DEFAULT "?",
    fpath TEXT NOT NULL,
    PRIMARY KEY (title, fpath),
    FOREIGN KEY (adder) REFERENCES users(id)
      ON DELETE RESTRICT
      ON UPDATE RESTRICT
);
