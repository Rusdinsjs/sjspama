import psycopg2
import os

DATABASE_URL = "postgres://root:rootpassword@localhost:5432/sjspama_dev"

def check_schema():
    conn = psycopg2.connect(DATABASE_URL)
    cur = conn.cursor()
    cur.execute("SELECT column_name FROM information_schema.columns WHERE table_name = 'employees'")
    columns = [row[0] for row in cur.fetchall()]
    print("Columns in 'employees' table:")
    for col in columns:
        print(f"- {col}")
    cur.close()
    conn.close()

if __name__ == "__main__":
    check_schema()
