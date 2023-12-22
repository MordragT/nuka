//! Derivation store synced between different peers

pub struct Store {
    // DAG of derivations
    // to prevent tampering with derivations hash must be sent alongside of derivaition and then the derivation must be hashed on the recv and compared with the sent hash, if equal add derivation to store
    derivations: Grax<Entry>,
}
