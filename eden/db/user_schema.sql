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

CREATE TABLE IF NOT EXISTS idprobs (
    id INTEGER PRIMARY KEY,
    prob REAL DEFAULT 0.0,
    remark TEXT DEFAULT ""
);

CREATE TABLE IF NOT EXISTS cards (
    csrc TEXT PRIMARY KEY,
    cname TEXT UNIQUE NOT NULL,
    crank INTEGER DEFAULT 3
      CHECK (crank > 0 AND crank < 6),
    element CHAR(5) 
      CHECK (element IN ('air','earth','fire','water')),
    atk INTEGER DEFAULT 0,
    lufa REAL DEFAULT 0.0,
    def REAL DEFAULT 0.0,
    lufd REAL DEFAULT 0.0,
    utl INTEGER DEFAULT 0,
    lufu REAL DEFAULT 0.0,
    subjct CHAR(30) NOT NULL,
    adder CHAR(30) NOT NULL,
    tradable INTEGER DEFAULT 1,
    FOREIGN KEY (subjct) REFERENCES users(id)
      ON DELETE RESTRICT
      ON UPDATE RESTRICT,
    FOREIGN KEY (adder) REFERENCES users(id)
      ON DELETE RESTRICT
      ON UPDATE RESTRICT
);

CREATE TABLE IF NOT EXISTS items (
    src TEXT UNIQUE NOT NULL,
    ownr CHAR(30) NOT NULL,
    lvl INTEGER DEFAULT 0,
    xp INTEGER DEFAULT 0,
    PRIMARY KEY (src, ownr),
    FOREIGN KEY (src) REFERENCES cards(csrc)
      ON DELETE CASCADE 
      ON UPDATE CASCADE,
    FOREIGN KEY (ownr) REFERENCES users(id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS tracks (
    title TEXT NOT NULL,
    artist TEXT DEFAULT "unknown",
    genre TEXT DEFAULT "unknown",
    leng INTEGER DEFAULT 0,
    adder CHAR(30) NOT NULL,
    fpath TEXT NOT NULL,
    PRIMARY KEY (title, fpath),
    FOREIGN KEY (adder) REFERENCES users(id)
      ON DELETE RESTRICT
      ON UPDATE RESTRICT
);
