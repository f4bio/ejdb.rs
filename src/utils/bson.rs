use bson::Bson;

pub trait BsonNumber {
    fn to_bson(self) -> Bson;
}

impl BsonNumber for f32 {
    #[inline]
    fn to_bson(self) -> Bson {
        Bson::from(self as f64)
    }
}

impl BsonNumber for f64 {
    #[inline]
    fn to_bson(self) -> Bson {
        Bson::from(self)
    }
}

impl BsonNumber for i32 {
    #[inline]
    fn to_bson(self) -> Bson {
        Bson::from(self)
    }
}

impl BsonNumber for i64 {
    #[inline]
    fn to_bson(self) -> Bson {
        Bson::from(self)
    }
}
