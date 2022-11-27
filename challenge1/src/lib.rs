use std::cmp::min;

fn xor_encryption(bytes: &[u8], cypher: u32, encrypt: bool) -> Vec<u8> {
    let mut payload = bytes;
    let size_payload = payload.len();
    if size_payload == 0 {
        panic!("no information in payloud");
    }
    assert!(!payload.is_empty());

    // create new vec to be return as a result
    let mut result = Vec::with_capacity(size_payload);

    //transform cypher from u32 to vec in order to be ready to iterate on it
    let mut current_cypher = cypher.to_be_bytes().to_vec();
    
    while size_payload > 0 {
        //get the min: 4(for 4 bytes) or those who have left(less than 4)
        let number = min(4, size_payload);

        // payload - current_payload - encrypted_bytes - rest
        // create a tuple
        let (current_payload, rest) = payload.split_at(number);

        // zip - an iterator that iterates two other iterators simultaneously
        // for every element apply ^ and colect the result in collection
        let encrypted_bytes: Vec<u8> = current_payload.iter()
                                                    .zip(current_cypher)
                                                    .map(|(byte,cypher)| byte ^ cypher)
                                                    .collect();
        current_cypher = if encrypt { encrypted_bytes.clone() }
                        else { current_payload.to_vec() };

        // include encpypted bytes in result vector
        result.extend(encrypted_bytes);
        // the rest ot bytes become on the next iteration the rest of payload
        payload = rest;
    }
    result
}

pub struct Packet {
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(source: &[u8]) -> Self {
        Self { payload: source.to_vec() }
    }

    pub fn encrypt(&self, cypher: u32) -> EncryptedPacket {
        EncryptedPacket { payload: xor_encryption(&self.payload, cypher, true) }
    }   
}

pub struct EncryptedPacket {
    pub payload: Vec<u8>,
}

impl EncryptedPacket {
    pub fn decrypt(&self, cypher: u32) -> Packet {
        Packet { payload: xor_encryption(&self.payload, cypher, false) }
    }
}

#[test]
    fn basic_test() {
        let packet = Packet::new(b"Secret message, please dont't hack");
        assert_eq!(packet.encrypt(0xDEADBEEF).decrypt(0xDEADBEEF).payload, packet.payload);
        assert_ne!(packet.encrypt(0xDEADBEF).decrypt(0xDEADBEEF).payload, packet.payload);
        assert_eq!(packet.encrypt(1024).decrypt(1024).payload, packet.payload);
    }
