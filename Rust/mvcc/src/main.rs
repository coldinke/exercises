use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::{atomic::AtomicU64, Arc, Mutex},
    vec,
};

pub type KVEngine = BTreeMap<Vec<u8>, Option<Vec<u8>>>;

static VERSION: AtomicU64 = AtomicU64::new(1);

fn acquire_next_version() -> u64 {
    let version = VERSION.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    version
}

lazy_static! {
    static ref ACTIVE_TXN: Arc<Mutex<HashMap<u64, Vec<Vec<u8>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct MVCC {
    kv: Arc<Mutex<KVEngine>>,
}

impl MVCC {
    pub fn new(kv: KVEngine) -> Self {
        Self {
            kv: Arc::new(Mutex::new(kv)),
        }
    }

    pub fn begin_transaction(&self) -> Transaction {
        Transaction::begin(self.kv.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Key {
    raw_key: Vec<u8>,
    version: u64,
}

impl Key {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

fn decode_key(b: &Vec<u8>) -> Key {
    bincode::deserialize(&b).unwrap()
}

pub struct Transaction {
    kv: Arc<Mutex<KVEngine>>,
    version: u64,
    active_xid: HashSet<u64>,
}

impl Transaction {
    pub fn begin(kv: Arc<Mutex<KVEngine>>) -> Self {
        let version = acquire_next_version();

        let mut active_txn = ACTIVE_TXN.lock().unwrap();
        let active_xid = active_txn.keys().cloned().collect();

        active_txn.insert(version, vec![]);

        Self {
            kv,
            version,
            active_xid,
        }
    }

    pub fn set(&self, key: &[u8], value: Vec<u8>) {
        self.write(key, Some(value))
    }

    pub fn delete(&self, key: &[u8]) {
        self.write(key, None)
    }

    fn write(&self, key: &[u8], value: Option<Vec<u8>>) {
        let mut kvegine = self.kv.lock().unwrap();
        for (enc_key, _) in kvegine.iter().rev() {
            let key_version = decode_key(enc_key);
            if key_version.raw_key.eq(key) {
                if !self.is_visible(key_version.version) {
                    panic!("Serialization error, try again.")
                }
                break;
            }
        }

        let mut active_txn = ACTIVE_TXN.lock().unwrap();
        active_txn
            .entry(self.version)
            .and_modify(|keys| keys.push(key.to_vec()))
            .or_insert_with(|| vec![key.to_vec()]);

        let enc_key = Key {
            raw_key: key.to_vec(),
            version: self.version,
        };

        kvegine.insert(enc_key.encode(), value);
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let kvengine = self.kv.lock().unwrap();
        for (k, v) in kvengine.iter().rev() {
            let key_version = decode_key(k);
            if key_version.raw_key.eq(key) && self.is_visible(key_version.version) {
                return v.clone();
            }
        }
        None
    }

    pub fn print_all(&self) {
        let mut records = BTreeMap::new();
        let kvengine = self.kv.lock().unwrap();
        for (k, v) in kvengine.iter() {
            let key_vesion = decode_key(k);
            if self.is_visible(key_vesion.version) {
                records.insert(key_vesion.raw_key, v.clone());
            }
        }

        for (k, v) in records.iter() {
            if let Some(value) = v {
                print!(
                    "{}={} ",
                    String::from_utf8_lossy(k),
                    String::from_utf8_lossy(value)
                );
            }
        }
        println!("");
    }

    pub fn commit(&self) {
        let mut active_txn = ACTIVE_TXN.lock().unwrap();
        active_txn.remove(&self.version);
    }

    pub fn rollback(&self) {
        let mut active_txn = ACTIVE_TXN.lock().unwrap();
        if let Some(keys) = active_txn.get(&self.version) {
            let mut kvengine = self.kv.lock().unwrap();
            for k in keys {
                let enc_key = Key {
                    raw_key: k.to_vec(),
                    version: self.version,
                };
                let res = kvengine.remove(&enc_key.encode());
                assert!(res.is_some());
            }
        }

        active_txn.remove(&self.version);
    }

    fn is_visible(&self, version: u64) -> bool {
        if self.active_xid.contains(&version) {
            return false;
        }
        version <= self.version
    }
}

fn main() {
    let eng = KVEngine::new();
    let mvcc = MVCC::new(eng);
    let tx0 = mvcc.begin_transaction();
    tx0.set(b"a", b"a1".to_vec());
    tx0.set(b"b", b"b1".to_vec());
    tx0.set(b"c", b"c1".to_vec());
    tx0.set(b"d", b"d1".to_vec());
    tx0.set(b"e", b"e1".to_vec());
    tx0.commit();

    let tx1 = mvcc.begin_transaction();
    tx1.set(b"a", b"a2".to_vec());
    tx1.set(b"e", b"e2".to_vec());

    tx1.print_all();

    let tx2 = mvcc.begin_transaction();
    tx2.delete(b"b");
    tx2.print_all();

    let tx3 = mvcc.begin_transaction();
    tx3.print_all();

    tx3.set(b"f", b"f1".to_vec());
    // panic. Serialization error, try again.
    tx2.set(b"f", b"f1".to_vec());
}
