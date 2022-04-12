use crate::{data::Position, Receiver, Result, Resume, Source, Value};

pub fn nullable<'src, T: Source<'src>>(nullable: Option<T>) -> Nullable<T> {
    Nullable::new(nullable)
}

pub struct Nullable<T> {
    nullable: Option<T>,
    position: Position,
}

impl<T> Nullable<T> {
    pub fn new(nullable: Option<T>) -> Self {
        Nullable {
            nullable,
            position: Position::Begin,
        }
    }
}

impl<'src, T: Source<'src>> Source<'src> for Nullable<T> {
    fn stream_resume<'data, R: Receiver<'data>>(&mut self, mut receiver: R) -> Result<Resume>
    where
        'src: 'data,
    {
        loop {
            match self.position {
                Position::Begin => {
                    receiver.nullable_begin()?;
                    self.position = Position::Value;
                }
                Position::Value => match self.nullable {
                    Some(ref mut v) => match v.stream_resume(&mut receiver)? {
                        Resume::Continue => return Ok(Resume::Continue),
                        Resume::Done => self.position = Position::End,
                    },
                    None => {
                        receiver.null()?;
                        self.position = Position::End;
                    }
                },
                Position::End => {
                    receiver.nullable_end()?;
                    self.position = Position::Done;
                }
                Position::Done => return Ok(Resume::Done),
            }
        }
    }

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(false)
    }
}

impl<T: Value> Value for Option<T> {
    fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
        match self {
            None => {
                receiver.nullable_begin()?;
                receiver.null()?;
                receiver.nullable_end()
            }
            Some(v) => {
                receiver.nullable_begin()?;
                v.stream(&mut receiver)?;
                receiver.nullable_end()
            }
        }
    }

    fn is_dynamic(&self) -> bool {
        false
    }

    fn to_bool(&self) -> Option<bool> {
        self.as_ref().and_then(|value| value.to_bool())
    }

    fn to_f32(&self) -> Option<f32> {
        self.as_ref().and_then(|value| value.to_f32())
    }

    fn to_f64(&self) -> Option<f64> {
        self.as_ref().and_then(|value| value.to_f64())
    }

    fn to_i8(&self) -> Option<i8> {
        self.as_ref().and_then(|value| value.to_i8())
    }

    fn to_i16(&self) -> Option<i16> {
        self.as_ref().and_then(|value| value.to_i16())
    }

    fn to_i32(&self) -> Option<i32> {
        self.as_ref().and_then(|value| value.to_i32())
    }

    fn to_i64(&self) -> Option<i64> {
        self.as_ref().and_then(|value| value.to_i64())
    }

    fn to_i128(&self) -> Option<i128> {
        self.as_ref().and_then(|value| value.to_i128())
    }

    fn to_u8(&self) -> Option<u8> {
        self.as_ref().and_then(|value| value.to_u8())
    }

    fn to_u16(&self) -> Option<u16> {
        self.as_ref().and_then(|value| value.to_u16())
    }

    fn to_u32(&self) -> Option<u32> {
        self.as_ref().and_then(|value| value.to_u32())
    }

    fn to_u64(&self) -> Option<u64> {
        self.as_ref().and_then(|value| value.to_u64())
    }

    fn to_u128(&self) -> Option<u128> {
        self.as_ref().and_then(|value| value.to_u128())
    }

    fn to_text(&self) -> Option<&str> {
        self.as_ref().and_then(|value| value.to_text())
    }

    fn to_binary(&self) -> Option<&[u8]> {
        self.as_ref().and_then(|value| value.to_binary())
    }
}
