use crate::{data::Position, Receiver, Result, Resume, Source};

pub fn map<'src, K: Source<'src>, V: Source<'src>, M: Iterator<Item = (K, V)>>(
    map: M,
) -> Map<K, V, M> {
    Map::new(map)
}

pub struct Map<K, V, M> {
    map: M,
    current: Option<KeyValue<K, V>>,
    position: Position,
}

enum KeyValue<K, V> {
    Key(K, Option<V>),
    Value(V),
}

impl<'src, K: Source<'src>, V: Source<'src>> KeyValue<K, V> {
    fn begin<'data, R: Receiver<'data>>(key: K, value: V, mut receiver: R) -> Result<Self>
    where
        'src: 'data,
    {
        receiver.map_key_begin()?;

        Ok(KeyValue::Key(key, Some(value)))
    }
}

impl<'src, K: Source<'src>, V: Source<'src>> Source<'src> for KeyValue<K, V> {
    fn stream_resume<'data, R: Receiver<'data>>(&mut self, mut receiver: R) -> Result<Resume>
    where
        'src: 'data,
    {
        match self {
            KeyValue::Key(key, value) => match key.stream_resume(&mut receiver)? {
                Resume::Continue => Ok(Resume::Continue),
                Resume::Done => {
                    receiver.map_key_end()?;
                    receiver.map_value_begin()?;

                    *self = KeyValue::Value(value.take().expect("missing value"));

                    Ok(Resume::Continue)
                }
            },
            KeyValue::Value(value) => match value.stream_resume(&mut receiver)? {
                Resume::Continue => Ok(Resume::Continue),
                Resume::Done => {
                    receiver.map_value_end()?;

                    Ok(Resume::Done)
                }
            },
        }
    }

    fn maybe_dynamic(&self) -> Option<bool> {
        Some(false)
    }
}

impl<K, V, M> Map<K, V, M> {
    pub fn new(map: M) -> Self {
        Map {
            map,
            position: Position::Begin,
            current: None,
        }
    }
}

impl<'src, K: Source<'src>, V: Source<'src>, M: Iterator<Item = (K, V)>> Source<'src>
    for Map<K, V, M>
{
    fn stream_resume<'data, R: Receiver<'data>>(&mut self, mut receiver: R) -> Result<Resume>
    where
        'src: 'data,
    {
        loop {
            if let Some(current) = self.current.as_mut() {
                match current.stream_resume(&mut receiver)? {
                    Resume::Continue => return Ok(Resume::Continue),
                    Resume::Done => self.current = None,
                }
            }

            debug_assert!(self.current.is_none());

            match self.position {
                Position::Begin => {
                    receiver.map_begin(None)?;
                    self.position = Position::Value;
                }
                Position::Value => match self.map.next() {
                    Some((key, value)) => {
                        self.current = Some(KeyValue::begin(key, value, &mut receiver)?)
                    }
                    None => self.position = Position::End,
                },
                Position::End => {
                    receiver.map_end()?;
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
