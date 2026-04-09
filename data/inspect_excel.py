import pandas as pd
import json

df = pd.read_excel('PT. SATRIA JAYA SULTRA - PAMA.xlsx', header=None)
header_row = -1
for i, row in df.iterrows():
    if any('NIK' in str(val).upper() for val in row):
        header_row = i
        break

if header_row != -1:
    df = pd.read_excel('PT. SATRIA JAYA SULTRA - PAMA.xlsx', skiprows=header_row)
    # Clean column names
    df.columns = [str(c).strip() for c in df.columns]
    print(json.dumps(df.columns.tolist()))
    # Print sample
    print(df.head(5).to_json(orient='records'))
else:
    print("Header not found")
