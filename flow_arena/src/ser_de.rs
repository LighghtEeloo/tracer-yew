use super::{Node, FlowArena};
use std::{fmt, hash::Hash};

#[cfg(feature = "serde1")]
use serde::ser::{Serialize, Serializer, SerializeStruct};
#[cfg(feature = "serde1")]
impl<Id: Serialize + Hash + Eq, Entity: Serialize> Serialize for FlowArena<Id, Entity> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut flow = serializer.serialize_struct("Flow", 2)?;
        flow.serialize_field("root", &self.root)?;
        let seq: Vec<&Node<Id, Entity>> = self.node_map.values().collect();
        flow.serialize_field("node_map", &seq)?;
        flow.end()
    }
}

#[cfg(feature = "serde1")]
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
#[cfg(feature = "serde1")]
use std::marker::PhantomData;
#[cfg(feature = "serde1")]
impl<'de, Id: Deserialize<'de> + Clone + Hash + Eq, Entity: Deserialize<'de>> Deserialize<'de> for FlowArena<Id, Entity> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        enum Field { Root, NodeMap }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Field, D::Error> {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`root` or `node_map`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "root" => Ok(Field::Root),
                            "node_map" => Ok(Field::NodeMap),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct FlowVisitor<Id: Hash + Eq, Entity> {
            marker: PhantomData<fn() -> FlowArena<Id, Entity>>
        }

        impl<'de, Id: Deserialize<'de> + Clone + Hash + Eq, Entity: Deserialize<'de>> Visitor<'de> for FlowVisitor<Id, Entity> {
            type Value = FlowArena<Id, Entity>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct FlowArena")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let root = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let node_vec: Vec<Node<Id, Entity>> = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let node_map = node_vec.into_iter().map(|node| (node.id().clone(), node)).collect();
                Ok(Self::Value { root, node_map })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut root = None;
                let mut node_map = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Root => {
                            if root.is_some() {
                                return Err(de::Error::duplicate_field("root"));
                            }
                            root = Some(map.next_value()?);
                        }
                        Field::NodeMap => {
                            if node_map.is_some() {
                                return Err(de::Error::duplicate_field("node_map"));
                            }
                            let node_vec: Vec<Node<Id, Entity>> = map.next_value()?;
                            node_map = Some(node_vec.into_iter().map(|node| (node.id().clone(), node)).collect());
                        }
                    }
                }
                let root = root.ok_or_else(|| de::Error::missing_field("root"))?;
                let node_map = node_map.ok_or_else(|| de::Error::missing_field("node_map"))?;
                Ok(Self::Value { root, node_map })
            }
        }

        const FIELDS: &'static [&'static str] = &["root", "node_map"];
        deserializer.deserialize_struct("Flow", FIELDS, FlowVisitor { marker: PhantomData })
    }
}