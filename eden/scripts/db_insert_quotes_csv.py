"""
A script intended for quickly rewriting quote data in the event of a schema change.

To export such a csv from a db file:
```
sqlite> .mode csv
sqlite> .once out.csv
sqlite> SELECT * FROM quotes;
sqlite> .quit
```
"""

import sqlite3
import os

script_path = os.path.realpath(__file__)
root_path = "/".join(script_path.split("/")[:-2])

conn = sqlite3.connect(f"{root_path}/db/user.db")
curs = conn.cursor()

fl = open(f"{root_path}/data/out.csv")

print("Inserting records.")
records = []
for line in fl.readlines():
	# Parse CSV by working backwards. We know there are 4 elements 
	# per line for the quotes schema, so there will be 3 commas, 
	# and the last 3 fields will not have any commas in them.
	idxs = []
	for i in range(len(line)-1, -1, -1):
		if len(idxs) < 3 and line[i] == ",":
			idxs.append(i)

	quote = line[1:idxs[2]-1]
	quotee = line[idxs[2]+1:idxs[1]]
	quoter = line[idxs[1]+1:idxs[0]]
	qweight = float(line[idxs[0]+1:-1])

	records.append((quote, quotee, quoter, qweight))

curs.executemany("INSERT INTO quotes VALUES(?, ?, ?, ?)", records)
conn.commit()
print("Done.")