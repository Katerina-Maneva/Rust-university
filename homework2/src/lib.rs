use std::fmt;
use std::convert::TryInto;
use std::cmp;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub enum PacketError {
    InvalidPacket,
    InvalidChecksum,
    UnknownProtocolVersion,
    CorruptedMessage,
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PacketError::InvalidPacket => write!(f, "InvalidPacket"),
            PacketError::InvalidChecksum => write!(f, "InvalidChecksum"),
            PacketError::UnknownProtocolVersion => write!(f, "UnknownProtocolVersion"),
            PacketError::CorruptedMessage => write!(f, "CorruptedMessage"),
        }
    }
}

impl std::error::Error for PacketError {}

#[derive(PartialEq, Debug)]
pub struct Packet<'a> {
   source_payload: &'a [u8],
}

impl<'a> Packet<'a> {
    
    pub fn from_source(source: &'a [u8], size: u8) -> (Self, &'a [u8]) {
        let size = min(size as usize, sourec.len());
        if size == 0 {
            panic!("no information in payloud");
        }
        let (payload, rest) = source.split_at(size);

        let source_size = source.len().try_into().unwrap();
        let payload_size = if source_size < size.try_into().unwrap() {
            source_size
        } else {
            size
        };
        assert!(payload_size <= source_size);
        (Packet{source_payload: payload}, rest)
    }
    


    /// Връща само slice-а който пакета опакова. Тоест, ако сме конструирали пакета със
    /// `Packet::from_source(b"abc", 3)`, очакваме `.payload()` да ни върне `b"abc"`.
    ///
   
    pub fn payload(&self) -> &[u8] {
        self.source_payload
    }

    /// Сериализира пакета, тоест превръща го в байтове, готови за трансфер. Версия, дължина,
    /// съобщение (payload), checksum.
    /// Последователно се вмъква инф. за пакета
    /// 
    
    pub fn serialize(&self) -> Vec<u8> {
        let size = self.source_payload.len();
        let mut output: Vec<u8> = Vec::with_capacity(size + 6);
        output.push(1); // version
        output.push(size); // lenght
        debug_assert!(!self.source_payload.is_empty());
        output.extend_from_slice(&self.source_payload);// payload
        let checksum: u32 = Self::checksum(self.source_payload);
        let bytes = checksum.to_be_bytes();
        output.extend_from_slice(&bytes);// checksum
        output
    }

    fn checksum(payload: &[u8]) -> u32 {
        payload.iter().map(|&b| b as u32).sum()
    }

    /// Имайки slice от байтове, искаме да извадим един пакет от началото и да върнем остатъка,
    /// пакетиран в `Result`.
    ///
    /// Байтовете са репрезентация на пакет -- версия, размер, и т.н. както е описано по-горе.
    ///
    /// Ако липсват версия, размер, чексума, или размера е твърде малък, за да може да се изпарси
    /// валиден пакет от байтовете, връщаме грешка `PacketError::InvalidPacket`.
    ///
    /// Ако версията е различна от 1, връщаме `PacketError::UnknownProtocolVersion`.
    ///
    /// Ако checksum-а, който прочитаме от последните 4 байта на пакета е различен от изчисления
    /// checksum на payload-а (сумата от байтовете му), връщаме `PacketError::InvalidChecksum`.
    ///
    /// Забележете, че ако размера е по-голям от истинския размер на payload-а, се очаква
    /// `PacketError::InvalidPacket`. Ако размера е по-малък от истинския размер на payload-а,
    /// въпросния ще се изпарси, но чексумата ще е грешна, така че ще очакваме
    /// `PacketError::InvalidChecksum`. Малко тъпо! Но уви, протоколите имат подобни тъпи ръбове,
    /// особено като са написани за един уикенд. Авторите обещават по-добър протокол за версия 2.
    ///
    pub fn deserialize(bytes: &[u8]) -> Result<(Packet, &[u8]), PacketError> {
        let size = bytes.len() az usize;
    // split_first -> Returns the first and all the rest of the elements of the slice, or None if it is empty.
    // ok_or_else -> Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err()).
    // prepare the required error
        let (&version, bytes) = bytes.split_first().ok_or_else(|| PacketError::InvalidPacket)?; 
        if version != 1 {
            Err(PacketError::InvalidPacket)
        } 
        
        let (&size, bytes) = bytes.split_first().ok_or_else(|| PacketError::InvalidPacket)?;
        if bytes.len() < size + 4 {
            Err(PacketError::InvalidPacket/)
        }
                
                let payload_size = bytes[1];";'
                ;"
                let index_byte_checksum = payload_size + 2;
                let index_byte_checksum_usize = index_byte_checksum as usize;
                let checksum = Self::checksum(&bytes[2..index_byte_checksum_usize]);
                if checksum != bytes[index_byte_checksum_usize].into() {
                    Err(PacketError::InvalidChecksum)
                } else {
                    Ok((Packet{source_payload: payload}, rest))
                }
           }
        }
        
    
}

/// Структура, която ще служи за итериране по пакети. Ще я конструираме от някакво съобщение, и
/// итерацията ще връща всеки следващ пакет, докато съобщението не бъде напълно "изпратено".
/// Изберете каквито полета ви трябват.
///
/// Може да е нужно да добавите lifetimes на дефиницията тук и/или на методите в impl блока.
///
/// 

pub struct PacketSerializer<'a> {
    source_payload: &'a [u8],
    packet_size: u8,
}

impl<'a> Iterator for PacketSerializer<'a> {
    
    type Item = Packet<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.source_payload.is_empty() {
            let result_from_source = Packet::from_source(self.source_payload, self.packet_size);
            self.source_payload = result_from_source.1; 
            Some(result_from_source.0)
        } else {
            assert_eq!(None, self.source_payload.iter().next());
            None
        }
        
    }
}

/// Този trait ще ни позволи да конвертираме един `String` (а ако искаме, и други неща) от и до
/// комплект от байтове за прехвърляне по мрежата.
///
/// Детайли за методите вижте по-долу в имплементацията на този trait за `String`.
///
pub trait Packetable: Sized {
    fn to_packets(&self, packet_size: u8) -> PacketSerializer;
    fn to_packet_data(&self, packet_size: u8) -> Vec<u8>;
    fn from_packet_data(packet_data: &[u8]) -> Result<Self, PacketError>;
}

impl Packetable for String {
    /// Този метод приема размер, който да използваме за размера на payload-а на всеки пакет. Връща
    /// итератор върху въпросните пакети. Низа трябва да се използва под формата на байтове.
    ///
    /// Както при `.from_source`, ако подадения `packet_size` е по-голям от дължината на оставащите
    /// байтове, приемаме, че размера на съответния пакет ще е колкото остава.
    ///
    fn to_packets(&self, packet_size: u8) -> PacketSerializer {
       PacketSerializer{source_payload: self.as_bytes(), packet_size: packet_size}
    }

    /// Имайки итератор по пакети, лесно можем да сериализираме всеки индивидуален пакет в поредица
    /// от байтове със `.serialize()` и да го натъпчем във вектора.
    ///
    /// Както при `.from_source`, ако подадения `packet_size` е по-голям от дължината на оставащите
    /// байтове, приемаме, че размера на съответния пакет ще е колкото остава.
    ///
    fn to_packet_data(&self, packet_size: u8) -> Vec<u8> {
        let packet_usize = packet_size as usize;
        let mut output: Vec<u8> = Vec::with_capacity(packet_usize); // TODO calculate capacity
        for packet in self.to_packets(packet_size) {
            output.append(&mut packet.serialize());
        }
        output
    }

    /// Обратното на горния метод е тази асоциирана функция -- имайки slice от байтове които са
    /// сериализирана репрезентация на пакети, искаме да десериализираме пакети от този slice, да
    /// им извадим payload-ите, и да ги сглобим в оригиналното съобщение.
    ///
    /// Грешките, които могат да се върнат, са същите, които идват от `.deserialize()`.
    ///
    /// Една допълнителна грешка, която може да се случи е при сглобяване на съобщението -- ако е
    /// имало липсващ пакет, може съчетанието на байтовете да не генерира правилно UTF8 съобщение.
    /// Тогава връщаме `PacketError::CorruptedMessage`.
    ///
    fn from_packet_data(mut packet_data: &[u8]) -> Result<Self, PacketError> {
        let mut result = String::new();
        while !packet_data.is_empty() {
            let packet = Packet::deserialize(packet_data);
            if packet.is_err() {
                return Err(packet.unwrap_err());
            } 
            let packet_val= packet.unwrap();
            result.push_str(&String::from_utf8_lossy(packet_val.0.payload()));
            packet_data = packet_val.1;
        }
        Ok(result)
    }
}
