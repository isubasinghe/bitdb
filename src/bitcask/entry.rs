pub enum DataType {
    Str,
    UInt32,
    Int32,
    UInt64, 
    Int64,
    Float,
    Double,
    Bytes,
}

pub struct Data<V> 

{
    timestamp: u64,
    keysize: u64, 
    valuesize: u64,
    key: String,
    value: V,
}




impl<V> Data<V> {
    fn new(key: String, value: V) -> Data<V> {
        Data{timestamp: 0, keysize: 0, valuesize: 0, key, value}
    }
}

