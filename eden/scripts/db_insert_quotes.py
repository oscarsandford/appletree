"""
This Python script helps automate populating the SQLite 
database with quotes that are preformatted in a JSON file.
"""

import sqlite3
import json
import os

script_path = os.path.realpath(__file__)
root_path = "/".join(script_path.split("/")[:-2])

conn = sqlite3.connect(f"{root_path}/db/user.db")
curs = conn.cursor()

fl = open(f"{root_path}/data/quotes.json")
data = json.load(fl)

print("Inserting records.")
records = []
for row in data:
	quote = row["quote_text"]
	quotee = row["quotee"]
	quoter = row["added_by"]
	records.append((quote, quotee, quoter, 0.5))

curs.executemany("INSERT INTO quotes VALUES(?, ?, ?, ?)", records)
conn.commit()
print("Done.")
