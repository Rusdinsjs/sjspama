import os
import pandas as pd
import requests
import json
import datetime

# Robust path handling to work from root or data folder
FILENAME = 'PT. SATRIA JAYA SULTRA - PAMA.xlsx'
if os.path.exists(FILENAME):
    FILE_PATH = FILENAME
elif os.path.exists(os.path.join('data', FILENAME)):
    FILE_PATH = os.path.join('data', FILENAME)
else:
    # Fallback/Error handling in import_data
    FILE_PATH = FILENAME
API_URL = 'http://127.0.0.1:8081/api/employees'

def parse_date(val):
    if pd.isna(val) or val == '-':
        return None
    if isinstance(val, datetime.datetime):
        return val.strftime('%Y-%m-%d')
    try:
        # Try DD/MM/YYYY
        return datetime.datetime.strptime(str(val), '%d/%m/%Y').strftime('%Y-%m-%d')
    except:
        return None

def import_data():
    print(f"Reading {FILE_PATH}...")
    df_raw = pd.read_excel(FILE_PATH, header=None)
    
    header_row = -1
    for i, row in df_raw.iterrows():
        if any('NIK' in str(val).upper() for val in row):
            header_row = i
            break
            
    if header_row == -1:
        print("Error: Could not find header row with 'NIK'")
        return

    df = pd.read_excel(FILE_PATH, skiprows=header_row)
    df.columns = [str(c).strip() for c in df.columns]
    
    success_count = 0
    error_count = 0

    for _, row in df.iterrows():
        # Map based on the observed sample columns
        # Note: Actual column names might vary slightly, using index-based and name-based fallback
        
        name = str(row.get('NAMA', row.iloc[3])).strip()
        if not name or name == 'nan' or name == '-':
            continue

        payload = {
            "nik": str(row.get('NIK', row.iloc[2])).strip() if not pd.isna(row.iloc[2]) else None,
            "name": name,
            "company": str(row.get('PERUSAHAAN', row.iloc[1])).strip(),
            "position": str(row.get('JABATAN', row.iloc[4])).strip(),
            "department": str(row.get('DEPARTMENT', row.iloc[5])).strip(),
            "status": "Active",
            "join_date": parse_date(row.iloc[22]), # Adjusted based on sample
            "contract_type": str(row.iloc[7]).strip() if not pd.isna(row.iloc[7]) else None,
            "ktp_number": str(row.iloc[8]).strip() if not pd.isna(row.iloc[8]) else None,
            "full_address": f"{row.iloc[9]}, {row.iloc[10]}" if not pd.isna(row.iloc[9]) else None,
            "village": str(row.iloc[11]).strip() if not pd.isna(row.iloc[11]) else None,
            "district": str(row.iloc[12]).strip() if not pd.isna(row.iloc[12]) else None,
            "city": str(row.iloc[13]).strip() if not pd.isna(row.iloc[13]) else None,
            "province": str(row.iloc[14]).strip() if not pd.isna(row.iloc[14]) else None,
            "origin_status": str(row.iloc[15]).strip() if not pd.isna(row.iloc[15]) else None,
            "gender": str(row.iloc[17]).strip() if not pd.isna(row.iloc[17]) else None,
            "marital_status": str(row.iloc[18]).strip() if not pd.isna(row.iloc[18]) else None,
            "religion": str(row.iloc[19]).strip() if not pd.isna(row.iloc[19]) else None,
            "birth_place": str(row.iloc[20]).strip() if not pd.isna(row.iloc[20]) else None,
            "birth_date": parse_date(row.iloc[21]),
            "education": str(row.iloc[26]).strip() if not pd.isna(row.iloc[26]) else None,
            "email": str(row.iloc[30]).strip() if not pd.isna(row.iloc[30]) else None,
            "phone_number": str(row.iloc[16]).strip() if not pd.isna(row.iloc[16]) else None,
            "simper_number": str(row.iloc[31]).strip() if not pd.isna(row.iloc[31]) else None,
            "simper_expiry": parse_date(row.iloc[32])
        }

        try:
            resp = requests.post(API_URL, json=payload)
            if resp.status_code == 201:
                success_count += 1
                print(f"Imported: {name}")
            else:
                error_count += 1
                print(f"Failed to import {name}: {resp.text}")
        except Exception as e:
            error_count += 1
            print(f"Connection error for {name}: {e}")

    print(f"\nImport Finished!")
    print(f"Success: {success_count}")
    print(f"Errors: {error_count}")

if __name__ == "__main__":
    import_data()
