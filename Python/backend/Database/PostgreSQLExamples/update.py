import psycopg2
from config import load_config

def update_vendor(vendor_id, vendor_name):
    