CREATE TABLE IF NOT EXISTS quotes (
    quote TEXT NOT NULL,
    quotee CHAR(20) NOT NULL,
    quoter CHAR(20) DEFAULT "?",
    qweight REAL DEFAULT 0.5,
    PRIMARY KEY (quote, quotee)
);

CREATE TABLE IF NOT EXISTS cards (
    id INTEGER PRIMARY KEY,
    ownr CHAR(20) NOT NULL,
    cid INTEGER NOT NULL,
    cname TEXT NOT NULL,
    crank INTEGER NOT NULL,
    clevel INTEGER DEFAULT 0,
    cimglink TEXT DEFAULT "",
    cdeck TEXT DEFAULT "unknown"
);

INSERT INTO quotes VALUES ("testing quote", "2348239423097", "82370945237904", 0.3);
INSERT INTO quotes VALUES ("another funny quote", "328463928", "32487293432", 0.7);