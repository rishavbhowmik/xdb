use std::collections::{BTreeMap, BTreeSet, HashMap};
use util::error::Error;

mod index_errors;

pub mod kv;

pub mod index_traits;

use index_traits::{IndexCloneTrait, IndexSerializationTrait, IndexTrait, UniqueIndexTrait};

// ... ... ... ... ... ... BTreeIndex ... ... ... ... ... ... ...

pub struct BTreeIndex {
    index: BTreeMap<kv::tuple::KeyData, BTreeSet<kv::tuple::ValueData>>,
}

impl IndexTrait<kv::tuple::KeyData, kv::tuple::ValueData> for BTreeIndex {
    fn get(&self, key: kv::tuple::KeyData) -> Vec<kv::tuple::ValueData> {
        let value_set = self.index.get(&key);
        match value_set {
            Some(value_set) => value_set.iter().cloned().collect(),
            None => Vec::new(),
        }
    }
    fn exists(&self, key: kv::tuple::KeyData, value: kv::tuple::ValueData) -> bool {
        let value_set = self.index.get(&key);
        match value_set {
            Some(value_set) => value_set.contains(&value),
            None => false,
        }
    }
    fn insert(
        &mut self,
        key: kv::tuple::KeyData,
        value: kv::tuple::ValueData,
    ) -> Result<Vec<u8>, Error> {
        self.index
            .entry(key.clone())
            .or_insert(BTreeSet::new())
            .insert(value.clone());
        let tuple = kv::tuple::KVTuple::new_insert(&key, &value);
        Ok(tuple.to_bytes())
    }
    fn remove(
        &mut self,
        key: kv::tuple::KeyData,
        value: kv::tuple::ValueData,
    ) -> Result<Vec<u8>, Error> {
        let value_set = self.index.get_mut(&key);
        match value_set {
            Some(value_set) => match value_set.remove(&value) {
                true => match value_set.is_empty() {
                    true => self.delete(key),
                    false => {
                        let tuple = kv::tuple::KVTuple::new_remove(&key, &value);
                        Ok(tuple.to_bytes())
                    }
                },
                false => Err(index_errors::index_trait_remove_value_not_found()),
            },
            None => Err(index_errors::index_trait_remove_key_not_found()),
        }
    }
    fn delete(&mut self, key: kv::tuple::KeyData) -> Result<Vec<u8>, Error> {
        match self.index.remove(&key) {
            Some(_) => Ok(kv::tuple::KVTuple::new_delete(&key).to_bytes()),
            None => Err(index_errors::index_trait_delete_key_not_found()),
        }
    }
}

impl IndexSerializationTrait<Self, BTreeMap<kv::tuple::KeyData, BTreeSet<kv::tuple::ValueData>>>
    for BTreeIndex
{
    /// Parse bytes and produce a new index.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut output = BTreeIndex {
            index: BTreeMap::new(),
        };

        let mut kv_map = kv::kv_tuples_from_bytes(bytes)?;

        while let Some(kv_tuple) = kv_map.pop_front() {
            let index_crud = kv_tuple.index_crud();
            match index_crud {
                kv::tuple::IndexCrud::DELETE => {
                    let key = kv_tuple.key();
                    match key {
                        Some(key) => {
                            output.delete(key)?;
                        }
                        None => {
                            return Err(index_errors::btree_index_from_bytes_delete_key_not_found())
                        }
                    }
                }
                kv::tuple::IndexCrud::INSERT => {
                    let key = kv_tuple.key();
                    let value = kv_tuple.value();
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            output.insert(key, value)?;
                        }
                        _ => {
                            return Err(
                                index_errors::btree_index_from_bytes_delete_key_or_value_not_found(
                                ),
                            )
                        }
                    }
                }
                kv::tuple::IndexCrud::REMOVE => {
                    let key = kv_tuple.key();
                    let value = kv_tuple.value();
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            if output.exists(key.clone(), value.clone()) {
                                output.remove(key, value)?;
                            }
                        }
                        _ => {
                            return Err(
                                index_errors::btree_index_from_bytes_delete_key_or_value_not_found(
                                ),
                            )
                        }
                    }
                }
                _ => continue,
            }
        }

        Ok(output)
    }

    /// Serialize the index into bytes.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (key, value_set) in self.index.iter() {
            for value in value_set.iter() {
                let tuple = kv::tuple::KVTuple::new_insert(key, value);
                bytes.append(&mut tuple.to_bytes());
            }
        }
        bytes
    }
}

impl IndexCloneTrait<Self, BTreeMap<kv::tuple::KeyData, BTreeSet<kv::tuple::ValueData>>>
    for BTreeIndex
{
    fn clone(&self) -> Self {
        BTreeIndex {
            index: self.index.clone(),
        }
    }

    fn index_clone(&self) -> BTreeMap<kv::tuple::KeyData, BTreeSet<kv::tuple::ValueData>> {
        self.index.clone()
    }
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

// ... ... ... ... ... ... UniqueBTreeIndex ... ... ... ... ... .

pub struct UniqueBTreeIndex {
    index: BTreeMap<kv::tuple::KeyData, kv::tuple::ValueData>,
}

impl UniqueIndexTrait<kv::tuple::KeyData, kv::tuple::ValueData> for UniqueBTreeIndex {
    fn get(&self, key: kv::tuple::KeyData) -> Option<kv::tuple::ValueData> {
        self.index.get(&key).cloned()
    }
    fn exists(&self, key: kv::tuple::KeyData, value: kv::tuple::ValueData) -> bool {
        match self.index.get(&key) {
            Some(value_data) => (*value_data) == value,
            None => false,
        }
    }
    fn delete(&mut self, key: kv::tuple::KeyData) -> Result<Vec<u8>, Error> {
        match self.index.remove(&key) {
            Some(_) => Ok(kv::tuple::KVTuple::new_delete(&key).to_bytes()),
            None => Err(index_errors::index_trait_delete_key_not_found()),
        }
    }
    fn set(
        &mut self,
        key: kv::tuple::KeyData,
        value: kv::tuple::ValueData,
        overwrite: bool,
    ) -> Result<Vec<u8>, Error> {
        match overwrite {
            true => {
                self.index.insert(key.clone(), value.clone());
                let tuple = kv::tuple::KVTuple::new_insert(&key, &value);
                return Ok(tuple.to_bytes());
            }
            false => {
                let entry = self.index.entry(key.clone());
                match entry {
                    std::collections::btree_map::Entry::Occupied(_) => {
                        return Err(index_errors::unique_index_trait_set_key_occupied());
                    }
                    std::collections::btree_map::Entry::Vacant(entry) => {
                        entry.insert(value.clone());
                        let tuple = kv::tuple::KVTuple::new_insert(&key, &value);
                        return Ok(tuple.to_bytes());
                    }
                }
            }
        }
    }
}

impl IndexSerializationTrait<Self, BTreeMap<kv::tuple::KeyData, kv::tuple::ValueData>>
    for UniqueBTreeIndex
{
    /// Parse bytes and produce a new index.
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut output = UniqueBTreeIndex {
            index: BTreeMap::new(),
        };

        let mut kv_map = kv::kv_tuples_from_bytes(bytes)?;

        while let Some(kv_tuple) = kv_map.pop_front() {
            let index_crud = kv_tuple.index_crud();
            match index_crud {
                kv::tuple::IndexCrud::DELETE => {
                    let key = kv_tuple.key();
                    match key {
                        Some(key) => {
                            output.delete(key)?;
                        }
                        None => {
                            return Err(
                                index_errors::unique_btree_index_from_bytes_delete_key_not_found(),
                            );
                        }
                    }
                }
                kv::tuple::IndexCrud::INSERT => {
                    let key = kv_tuple.key();
                    let value = kv_tuple.value();
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            output.set(key, value, true)?;
                        }
                        _ => {
                            return Err(index_errors::unique_btree_index_from_bytes_insert_key_or_value_not_found());
                        }
                    }
                }
                kv::tuple::IndexCrud::REMOVE => {
                    let key = kv_tuple.key();
                    let value = kv_tuple.value();
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            if output.exists(key.clone(), value) {
                                output.delete(key)?;
                            }
                        }
                        _ => {
                            return Err(index_errors::unique_btree_index_from_bytes_remove_key_or_value_not_found());
                        }
                    }
                }
                _ => continue,
            }
        }

        Ok(output)
    }

    /// Serialize the index to bytes.
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (key, value) in self.index.iter() {
            let tuple = kv::tuple::KVTuple::new_insert(key, value);
            bytes.append(&mut tuple.to_bytes());
        }
        bytes
    }
}

impl IndexCloneTrait<Self, BTreeMap<kv::tuple::KeyData, kv::tuple::ValueData>>
    for UniqueBTreeIndex
{
    fn clone(&self) -> Self {
        UniqueBTreeIndex {
            index: self.index.clone(),
        }
    }

    fn index_clone(&self) -> BTreeMap<kv::tuple::KeyData, kv::tuple::ValueData> {
        self.index.clone()
    }
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

// ... ... ... ... ... ... HashMapIndex ... ... ... ... ... ... .

pub struct HashMapIndex {
    index: HashMap<kv::tuple::KeyData, BTreeSet<kv::tuple::ValueData>>,
}

impl IndexTrait<kv::tuple::KeyData, kv::tuple::ValueData> for HashMapIndex {
    fn get(&self, key: kv::tuple::KeyData) -> Vec<kv::tuple::ValueData> {
        let mut output = Vec::new();
        if let Some(value_set) = self.index.get(&key) {
            for value in value_set.iter() {
                output.push(value.clone());
            }
        }
        output
    }
    fn exists(&self, key: kv::tuple::KeyData, value: kv::tuple::ValueData) -> bool {
        match self.index.get(&key) {
            Some(value_set) => value_set.contains(&value),
            None => false,
        }
    }
    fn delete(&mut self, key: kv::tuple::KeyData) -> Result<Vec<u8>, Error> {
        match self.index.remove(&key) {
            Some(_) => Ok(kv::tuple::KVTuple::new_delete(&key).to_bytes()),
            None => Err(index_errors::index_trait_delete_key_not_found()),
        }
    }
    fn insert(
        &mut self,
        key: kv::tuple::KeyData,
        value: kv::tuple::ValueData,
    ) -> Result<Vec<u8>, Error> {
        self.index
            .entry(key.clone())
            .or_insert_with(|| BTreeSet::new())
            .insert(value.clone());
        let tuple = kv::tuple::KVTuple::new_insert(&key, &value);
        return Ok(tuple.to_bytes());
    }
    fn remove(
        &mut self,
        key: kv::tuple::KeyData,
        value: kv::tuple::ValueData,
    ) -> Result<Vec<u8>, Error> {
        match self.index.remove(&key) {
            Some(_) => Ok(kv::tuple::KVTuple::new_remove(&key, &value).to_bytes()),
            None => Err(index_errors::index_trait_remove_key_not_found()),
        }
    }
}

impl HashMapIndex {
    /// Parse bytes and produce a new index.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut output = HashMapIndex {
            index: HashMap::new(),
        };

        let mut kv_map = kv::kv_tuples_from_bytes(bytes)?;

        while let Some(kv_tuple) = kv_map.pop_front() {
            let index_crud = kv_tuple.index_crud();
            match index_crud {
                kv::tuple::IndexCrud::DELETE => {
                    let key = kv_tuple.key();
                    match key {
                        Some(key) => {
                            output.delete(key)?;
                        }
                        None => {
                            return Err(
                                index_errors::hash_map_index_from_bytes_delete_key_not_found(),
                            );
                        }
                    }
                }
                kv::tuple::IndexCrud::INSERT => {
                    let key = kv_tuple.key();
                    let value = kv_tuple.value();
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            output.insert(key, value)?;
                        }
                        _ => {
                            return Err(index_errors::hash_map_index_from_bytes_delete_key_or_value_not_found());
                        }
                    }
                }
                kv::tuple::IndexCrud::REMOVE => {
                    let key = kv_tuple.key();
                    let value = kv_tuple.value();
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            if output.exists(key.clone(), value.clone()) {
                                output.remove(key, value)?;
                            }
                        }
                        _ => {
                            return Err(index_errors::hash_map_index_from_bytes_remove_key_or_value_not_found());
                        }
                    }
                }
                _ => continue,
            }
        }

        Ok(output)
    }

    /// Serialize the index to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (key, value_set) in self.index.iter() {
            for value in value_set.iter() {
                let tuple = kv::tuple::KVTuple::new_insert(key, value);
                bytes.append(&mut tuple.to_bytes());
            }
        }
        bytes
    }
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

// ... ... ... ... ... ... UniqueHashMapIndex ... ... ... ... ...

pub struct UniqueHashMapIndex {
    index: HashMap<kv::tuple::KeyData, kv::tuple::ValueData>,
}

impl UniqueIndexTrait<kv::tuple::KeyData, kv::tuple::ValueData> for UniqueHashMapIndex {
    fn get(&self, key: kv::tuple::KeyData) -> Option<kv::tuple::ValueData> {
        self.index.get(&key).cloned()
    }
    fn exists(&self, key: kv::tuple::KeyData, value: kv::tuple::ValueData) -> bool {
        match self.index.get(&key) {
            Some(value_set) => (*value_set) == value,
            None => false,
        }
    }
    fn delete(&mut self, key: kv::tuple::KeyData) -> Result<Vec<u8>, Error> {
        match self.index.remove(&key) {
            Some(_) => Ok(kv::tuple::KVTuple::new_delete(&key).to_bytes()),
            None => Err(index_errors::index_trait_delete_key_not_found()),
        }
    }
    fn set(
        &mut self,
        key: kv::tuple::KeyData,
        value: kv::tuple::ValueData,
        overwrite: bool,
    ) -> Result<Vec<u8>, Error> {
        match overwrite {
            true => {
                self.index.insert(key.clone(), value.clone());
                let tuple = kv::tuple::KVTuple::new_insert(&key, &value);
                return Ok(tuple.to_bytes());
            }
            false => {
                let entry = self.index.entry(key.clone());
                match entry {
                    std::collections::hash_map::Entry::Occupied(_) => {
                        return Err(index_errors::unique_index_trait_set_key_occupied());
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(value.clone());
                        let tuple = kv::tuple::KVTuple::new_insert(&key, &value);
                        return Ok(tuple.to_bytes());
                    }
                }
            }
        }
    }
}

impl UniqueHashMapIndex {
    pub fn new() -> UniqueHashMapIndex {
        UniqueHashMapIndex {
            index: HashMap::new(),
        }
    }
    /// Parse bytes and produce a new index.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut output = UniqueHashMapIndex {
            index: HashMap::new(),
        };

        let mut kv_map = kv::kv_tuples_from_bytes(bytes)?;

        while let Some(kv_tuple) = kv_map.pop_front() {
            match kv_tuple.index_crud() {
                kv::tuple::IndexCrud::DELETE => {
                    let key = kv_tuple.key();
                    match key {
                        Some(key) => {
                            output.delete(key)?;
                        }
                        None => {
                            return Err(
                                index_errors::unique_hash_map_index_from_bytes_delete_key_not_found(
                                ),
                            );
                        }
                    }
                }
                kv::tuple::IndexCrud::INSERT => {
                    let key = kv_tuple.key();
                    let value = kv_tuple.value();
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            output.set(key, value, true)?;
                        }
                        _ => {
                            return Err(index_errors::unique_hash_map_index_from_bytes_insert_key_or_value_not_found());
                        }
                    }
                }
                kv::tuple::IndexCrud::REMOVE => {
                    let key = kv_tuple.key();
                    let value = kv_tuple.value();
                    match (key, value) {
                        (Some(key), Some(value)) => {
                            if output.exists(key.clone(), value) {
                                output.delete(key)?;
                            }
                        }
                        _ => {
                            return Err(index_errors::unique_hash_map_index_from_bytes_remove_key_or_value_not_found());
                        }
                    }
                }
                _ => continue,
            }
        }

        Ok(output)
    }

    /// Serialize the index to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for (key, value) in self.index.iter() {
            let tuple = kv::tuple::KVTuple::new_insert(key, value);
            bytes.append(&mut tuple.to_bytes());
        }
        bytes
    }
}

// ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

#[cfg(test)]
mod tests {

    use super::*;

    // ... ... ... ... ... ... BTreeIndex ... ... ... ... ... ... ...

    #[test]
    fn btree_index_new() {
        let mut btree_index = BTreeIndex::from_bytes(&[]).unwrap();
        assert_eq!(btree_index.index.len(), 0);

        let k0 = vec![];
        let v0 = vec![];
        let sync_bytes = btree_index.insert(k0.clone(), v0.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 1);
        assert_eq!(btree_index.exists(k0.clone(), v0.clone()), true);
        assert_eq!(btree_index.get(k0.clone()), vec![v0.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k0, &v0).to_bytes()
        );

        // insert some keys & values

        let (k1, v1) = (vec![1], vec![1]);
        let sync_bytes = btree_index.insert(k1.clone(), v1.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 2);
        assert_eq!(btree_index.exists(k1.clone(), v1.clone()), true);
        assert_eq!(btree_index.get(k1.clone()), vec![v1.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k1, &v1).to_bytes()
        );

        let v1_2 = vec![1, 1];
        let sync_bytes = btree_index.insert(k1.clone(), v1_2.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 2);
        assert_eq!(btree_index.exists(k1.clone(), v1_2.clone()), true);
        assert_eq!(btree_index.get(k1.clone()), vec![v1.clone(), v1_2.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k1, &v1_2).to_bytes()
        );

        let (k2, v2) = (vec![2, 2], vec![2]);
        let sync_bytes = btree_index.insert(k2.clone(), v2.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 3);
        assert_eq!(btree_index.exists(k2.clone(), v2.clone()), true);
        assert_eq!(btree_index.get(k2.clone()), vec![v2.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k2, &v2).to_bytes()
        );

        let v2_2 = vec![2, 2];
        let sync_bytes = btree_index.insert(k2.clone(), v2_2.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 3);
        assert_eq!(btree_index.exists(k2.clone(), v2_2.clone()), true);
        assert_eq!(btree_index.get(k2.clone()), vec![v2.clone(), v2_2.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k2, &v2_2).to_bytes()
        );

        let v2_3 = vec![2, 2, 2];
        let sync_bytes = btree_index.insert(k2.clone(), v2_3.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 3);
        assert_eq!(btree_index.exists(k2.clone(), v2_3.clone()), true);
        assert_eq!(
            btree_index.get(k2.clone()),
            vec![v2.clone(), v2_2.clone(), v2_3.clone()]
        );
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k2, &v2_3).to_bytes()
        );

        let (k3, v3) = (vec![3, 3, 3], vec![3]);
        let sync_bytes = btree_index.insert(k3.clone(), v3.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 4);
        assert_eq!(btree_index.exists(k3.clone(), v3.clone()), true);
        assert_eq!(btree_index.get(k3.clone()), vec![v3.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k3, &v3).to_bytes()
        );

        let v3_2 = vec![3, 3, 3];
        let sync_bytes = btree_index.insert(k3.clone(), v3_2.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 4);
        assert_eq!(btree_index.exists(k3.clone(), v3_2.clone()), true);
        assert_eq!(btree_index.get(k3.clone()), vec![v3.clone(), v3_2.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k3, &v3_2).to_bytes()
        );

        let (k4, v4) = (vec![4, 4, 4, 4], vec![4]);
        let sync_bytes = btree_index.insert(k4.clone(), v4.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 5);
        assert_eq!(btree_index.exists(k4.clone(), v4.clone()), true);
        assert_eq!(btree_index.get(k4.clone()), vec![v4.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k4, &v4).to_bytes()
        );

        // remove from key-value pairs (remove value)

        let sync_bytes = btree_index.remove(k1.clone(), v1_2.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 5);
        assert_eq!(btree_index.exists(k1.clone(), v1_2.clone()), false);
        assert_eq!(btree_index.get(k1.clone()), vec![v1.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_remove(&k1, &v1_2).to_bytes()
        );

        let sync_bytes = btree_index.remove(k2.clone(), v2_3.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 5);
        assert_eq!(btree_index.exists(k2.clone(), v2_3.clone()), false);
        assert_eq!(btree_index.get(k2.clone()), vec![v2.clone(), v2_2.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_remove(&k2, &v2_3).to_bytes()
        );

        let sync_bytes = btree_index.remove(k0.clone(), v0.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 4);
        assert_eq!(btree_index.exists(k0.clone(), v0.clone()), false);
        assert_eq!(btree_index.get(k0.clone()), vec![] as Vec<Vec<u8>>);
        assert_eq!(sync_bytes, kv::tuple::KVTuple::new_delete(&k0).to_bytes());

        // remove unavailable key-value pairs (remove value)

        let (random_k, random_v) = (vec![25], vec![25]);
        let result = btree_index.remove(random_k.clone(), random_v.clone());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), "index_trait_remove_key_not_found");

        let result = btree_index.remove(k1.clone(), v1_2.clone());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), "index_trait_remove_value_not_found");

        let result = btree_index.remove(k0.clone(), v0.clone());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), "index_trait_remove_key_not_found");

        // insert some key value pairs again (insert value)

        let v3_3 = vec![3, 3, 3];
        let sync_bytes = btree_index.insert(k3.clone(), v3_3.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 4);
        assert_eq!(btree_index.exists(k3.clone(), v3_3.clone()), true);
        assert_eq!(btree_index.get(k3.clone()), vec![v3.clone(), v3_3.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k3, &v3_3).to_bytes()
        );

        let v4_2 = vec![4, 4];
        let sync_bytes = btree_index.insert(k4.clone(), v4_2.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 4);
        assert_eq!(btree_index.exists(k4.clone(), v4_2.clone()), true);
        assert_eq!(btree_index.get(k4.clone()), vec![v4.clone(), v4_2.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k4, &v4_2).to_bytes()
        );

        let v4_3 = vec![4, 4, 4];
        let sync_bytes = btree_index.insert(k4.clone(), v4_3.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 4);
        assert_eq!(btree_index.exists(k4.clone(), v4_3.clone()), true);
        assert_eq!(
            btree_index.get(k4.clone()),
            vec![v4.clone(), v4_2.clone(), v4_3.clone()]
        );
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k4, &v4_3).to_bytes()
        );

        let v4_4 = vec![4, 4, 4, 4];
        let sync_bytes = btree_index.insert(k4.clone(), v4_4.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 4);
        assert_eq!(btree_index.exists(k4.clone(), v4_4.clone()), true);
        assert_eq!(
            btree_index.get(k4.clone()),
            vec![v4.clone(), v4_2.clone(), v4_3.clone(), v4_4.clone()]
        );
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_insert(&k4, &v4_4).to_bytes()
        );

        // delete some keys

        let sync_bytes = btree_index.delete(k1.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 3);
        assert_eq!(btree_index.exists(k1.clone(), v1_2.clone()), false);
        assert_eq!(btree_index.get(k1.clone()), vec![] as Vec<Vec<u8>>);
        assert_eq!(sync_bytes, kv::tuple::KVTuple::new_delete(&k1).to_bytes());

        let sync_bytes = btree_index.delete(k2.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 2);
        assert_eq!(btree_index.exists(k2.clone(), v2_3.clone()), false);
        assert_eq!(btree_index.get(k2.clone()), vec![] as Vec<Vec<u8>>);
        assert_eq!(sync_bytes, kv::tuple::KVTuple::new_delete(&k2).to_bytes());

        let sync_bytes = btree_index.delete(k3.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 1);
        assert_eq!(btree_index.exists(k3.clone(), v3_3.clone()), false);
        assert_eq!(btree_index.get(k3.clone()), vec![] as Vec<Vec<u8>>);
        assert_eq!(sync_bytes, kv::tuple::KVTuple::new_delete(&k3).to_bytes());

        // delete unavailable keys

        let random_k = vec![25];
        let result = btree_index.delete(random_k.clone());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), "index_trait_delete_key_not_found");
        assert_eq!(btree_index.index.len(), 1);

        let result = btree_index.delete(k0.clone());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code(), "index_trait_delete_key_not_found");
        assert_eq!(btree_index.index.len(), 1);

        // delete key by removing all values

        let sync_bytes = btree_index.remove(k4.clone(), v4.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 1);
        assert_eq!(btree_index.exists(k4.clone(), v4.clone()), false);
        assert_eq!(
            btree_index.get(k4.clone()),
            vec![v4_2.clone(), v4_3.clone(), v4_4.clone()]
        );
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_remove(&k4, &v4).to_bytes()
        );

        let sync_bytes = btree_index.remove(k4.clone(), v4_2.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 1);
        assert_eq!(btree_index.exists(k4.clone(), v4_2.clone()), false);
        assert_eq!(
            btree_index.get(k4.clone()),
            vec![v4_3.clone(), v4_4.clone()]
        );
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_remove(&k4, &v4_2).to_bytes()
        );

        let sync_bytes = btree_index.remove(k4.clone(), v4_3.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 1);
        assert_eq!(btree_index.exists(k4.clone(), v4_3.clone()), false);
        assert_eq!(btree_index.get(k4.clone()), vec![v4_4.clone()]);
        assert_eq!(
            sync_bytes,
            kv::tuple::KVTuple::new_remove(&k4, &v4_3).to_bytes()
        );

        let sync_bytes = btree_index.remove(k4.clone(), v4_4.clone()).unwrap();
        assert_eq!(btree_index.index.len(), 0);
        assert_eq!(btree_index.exists(k4.clone(), v4_4.clone()), false);
        assert_eq!(btree_index.get(k4.clone()), vec![] as Vec<Vec<u8>>);
        assert_eq!(sync_bytes, kv::tuple::KVTuple::new_delete(&k4).to_bytes());
    }

    #[test]
    fn btree_index_from_bytes_and_to_back() {
        let kv_item_list = [
            ("One".as_bytes().to_vec(), "एक".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "दो".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "२".as_bytes().to_vec()),
            ("Three".as_bytes().to_vec(), "तीन".as_bytes().to_vec()),
            ("Three".as_bytes().to_vec(), "३".as_bytes().to_vec()),
            ("Three".as_bytes().to_vec(), "તીન".as_bytes().to_vec()),
            ("Four".as_bytes().to_vec(), "चार".as_bytes().to_vec()),
            ("Four".as_bytes().to_vec(), "४".as_bytes().to_vec()),
            ("Four".as_bytes().to_vec(), "ચાર".as_bytes().to_vec()),
            ("Five".as_bytes().to_vec(), "पाँच".as_bytes().to_vec()),
            ("Five".as_bytes().to_vec(), "५".as_bytes().to_vec()),
            ("Five".as_bytes().to_vec(), "પાઁચ".as_bytes().to_vec()),
        ];
        let mut btree_index: BTreeIndex = BTreeIndex::from_bytes(&[]).unwrap();
        for kv_item in kv_item_list.iter() {
            let insert_result = btree_index.insert(kv_item.0.clone(), kv_item.1.clone());
            assert!(insert_result.is_ok());
        }

        let btree_index = btree_index; // immutable

        // verify the btree_index is as expected
        {
            // key: "One"
            let value_set = btree_index.get("One".as_bytes().to_vec());
            assert_eq!(value_set, ["एक".as_bytes().to_vec()]);

            // key: "Two"
            let value_set = btree_index.get("Two".as_bytes().to_vec());
            assert_eq!(
                value_set,
                ["दो".as_bytes().to_vec(), "२".as_bytes().to_vec()]
            );

            // key: "Three"
            let value_set = btree_index.get("Three".as_bytes().to_vec());
            assert_eq!(
                value_set,
                [
                    "तीन".as_bytes().to_vec(),
                    "३".as_bytes().to_vec(),
                    "તીન".as_bytes().to_vec()
                ]
            );

            // key: "Four"
            let value_set = btree_index.get("Four".as_bytes().to_vec());
            assert_eq!(
                value_set,
                [
                    "चार".as_bytes().to_vec(),
                    "४".as_bytes().to_vec(),
                    "ચાર".as_bytes().to_vec()
                ]
            );

            // key: "Five"
            let value_set = btree_index.get("Five".as_bytes().to_vec());
            assert_eq!(
                value_set,
                [
                    "पाँच".as_bytes().to_vec(),
                    "५".as_bytes().to_vec(),
                    "પાઁચ".as_bytes().to_vec()
                ]
            );
        }

        // rebuild index from bytes
        {
            let bytes = btree_index.to_bytes();
            let result = BTreeIndex::from_bytes(&bytes);
            assert!(result.is_ok());
            let parsed_btree_index = result.unwrap();

            for kv_item in kv_item_list.iter() {
                assert_eq!(
                    parsed_btree_index.get(kv_item.0.clone()),
                    btree_index.get(kv_item.0.clone())
                );
            }
        }
    }

    // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

    // ... ... ... ... ... ... UniqueBTreeIndex ... ... ... ... ... .

    #[test]
    fn unique_index_from_bytes_and_to_back() {
        let kv_item_list = [
            ("One".as_bytes().to_vec(), "एक".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "2".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "Do".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "दो".as_bytes().to_vec()),
            ("Three".as_bytes().to_vec(), "तीन".as_bytes().to_vec()),
            ("Four".as_bytes().to_vec(), "चार".as_bytes().to_vec()),
            ("Five".as_bytes().to_vec(), "पाँच".as_bytes().to_vec()),
        ];
        let mut unique_index: UniqueBTreeIndex = UniqueBTreeIndex::from_bytes(&[]).unwrap();
        for kv_item in kv_item_list.iter() {
            let insert_result = unique_index.set(kv_item.0.clone(), kv_item.1.clone(), true);
            assert!(insert_result.is_ok());
        }

        let btree_index = unique_index; // immutable

        // verify the btree_index is as expected
        {
            // key: "One"
            let value = btree_index.get("One".as_bytes().to_vec());
            assert_eq!(value, Some("एक".as_bytes().to_vec()));
            // key: "Two"
            let value = btree_index.get("Two".as_bytes().to_vec());
            assert_eq!(value, Some("दो".as_bytes().to_vec()));
            // key: "Three"
            let value = btree_index.get("Three".as_bytes().to_vec());
            assert_eq!(value, Some("तीन".as_bytes().to_vec()));
            // key: "Four"
            let value = btree_index.get("Four".as_bytes().to_vec());
            assert_eq!(value, Some("चार".as_bytes().to_vec()));
            // key: "Five"
            let value = btree_index.get("Five".as_bytes().to_vec());
            assert_eq!(value, Some("पाँच".as_bytes().to_vec()));
        }

        // rebuild index from bytes
        {
            let bytes = btree_index.to_bytes();
            let result = UniqueBTreeIndex::from_bytes(&bytes);
            assert!(result.is_ok());
            let parsed_unique_btree_index = result.unwrap();

            for kv_item in kv_item_list.iter() {
                assert_eq!(
                    parsed_unique_btree_index.get(kv_item.0.clone()),
                    btree_index.get(kv_item.0.clone())
                );
            }
        }
    }

    // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

    // ... ... ... ... ... ... HashMapIndex ... ... ... ... ... ... .

    #[test]
    fn hash_map_index_from_bytes_and_to_back() {
        let kv_item_list = [
            ("One".as_bytes().to_vec(), "एक".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "दो".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "२".as_bytes().to_vec()),
            ("Three".as_bytes().to_vec(), "तीन".as_bytes().to_vec()),
            ("Three".as_bytes().to_vec(), "३".as_bytes().to_vec()),
            ("Three".as_bytes().to_vec(), "તીન".as_bytes().to_vec()),
            ("Four".as_bytes().to_vec(), "चार".as_bytes().to_vec()),
            ("Four".as_bytes().to_vec(), "४".as_bytes().to_vec()),
            ("Four".as_bytes().to_vec(), "ચાર".as_bytes().to_vec()),
            ("Five".as_bytes().to_vec(), "पाँच".as_bytes().to_vec()),
            ("Five".as_bytes().to_vec(), "५".as_bytes().to_vec()),
            ("Five".as_bytes().to_vec(), "પાઁચ".as_bytes().to_vec()),
        ];
        let mut btree_index = HashMapIndex::from_bytes(&[]).unwrap();
        for kv_item in kv_item_list.iter() {
            let insert_result = btree_index.insert(kv_item.0.clone(), kv_item.1.clone());
            assert!(insert_result.is_ok());
        }

        let btree_index = btree_index; // immutable

        // verify the btree_index is as expected
        {
            // key: "One"
            let value_set = btree_index.get("One".as_bytes().to_vec());
            assert_eq!(value_set, ["एक".as_bytes().to_vec(),]);

            // key: "Two"
            let value_set = btree_index.get("Two".as_bytes().to_vec());
            assert_eq!(
                value_set,
                ["दो".as_bytes().to_vec(), "२".as_bytes().to_vec(),]
            );

            // key: "Three"
            let value_set = btree_index.get("Three".as_bytes().to_vec());
            assert_eq!(
                value_set,
                [
                    "तीन".as_bytes().to_vec(),
                    "३".as_bytes().to_vec(),
                    "તીન".as_bytes().to_vec(),
                ]
            );

            // key: "Four"
            let value_set = btree_index.get("Four".as_bytes().to_vec());
            assert_eq!(
                value_set,
                [
                    "चार".as_bytes().to_vec(),
                    "४".as_bytes().to_vec(),
                    "ચાર".as_bytes().to_vec(),
                ]
            );

            // key: "Five"
            let value_set = btree_index.get("Five".as_bytes().to_vec());
            assert_eq!(
                value_set,
                [
                    "पाँच".as_bytes().to_vec(),
                    "५".as_bytes().to_vec(),
                    "પાઁચ".as_bytes().to_vec(),
                ]
            );
        }

        // rebuild index from bytes
        {
            let bytes = btree_index.to_bytes();
            let result = BTreeIndex::from_bytes(&bytes);
            assert!(result.is_ok());
            let parsed_btree_index = result.unwrap();

            for kv_item in kv_item_list.iter() {
                assert_eq!(
                    parsed_btree_index.get(kv_item.0.clone()),
                    btree_index.get(kv_item.0.clone())
                );
            }
        }
    }

    // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..

    // ... ... ... ... ... ... UniqueHashMapIndex ... ... ... ... ...

    #[test]
    fn unique_hash_map_index_from_bytes_and_to_back() {
        let kv_item_list = [
            ("One".as_bytes().to_vec(), "एक".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "2".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "Do".as_bytes().to_vec()),
            ("Two".as_bytes().to_vec(), "दो".as_bytes().to_vec()),
            ("Three".as_bytes().to_vec(), "तीन".as_bytes().to_vec()),
            ("Four".as_bytes().to_vec(), "चार".as_bytes().to_vec()),
            ("Five".as_bytes().to_vec(), "पाँच".as_bytes().to_vec()),
        ];
        let mut unique_index: UniqueHashMapIndex = UniqueHashMapIndex::from_bytes(&[]).unwrap();
        for kv_item in kv_item_list.iter() {
            let insert_result = unique_index.set(kv_item.0.clone(), kv_item.1.clone(), true);
            assert!(insert_result.is_ok());
        }

        let btree_index = unique_index; // immutable

        // verify the btree_index is as expected
        {
            // key: "One"
            let value = btree_index.get("One".as_bytes().to_vec());
            assert_eq!(value, Some("एक".as_bytes().to_vec()));

            // key: "Two"
            let value = btree_index.get("Two".as_bytes().to_vec());
            assert_eq!(value, Some("दो".as_bytes().to_vec()));

            // key: "Three"
            let value = btree_index.get("Three".as_bytes().to_vec());
            assert_eq!(value, Some("तीन".as_bytes().to_vec()));

            // key: "Four"
            let value = btree_index.get("Four".as_bytes().to_vec());
            assert_eq!(value, Some("चार".as_bytes().to_vec()));

            // key: "Five"
            let value = btree_index.get("Five".as_bytes().to_vec());
            assert_eq!(value, Some("पाँच".as_bytes().to_vec()));
        }

        // rebuild index from bytes
        {
            let bytes = btree_index.to_bytes();
            let result = UniqueHashMapIndex::from_bytes(&bytes);
            assert!(result.is_ok());
            let parsed_btree_index = result.unwrap();

            for kv_item in kv_item_list.iter() {
                assert_eq!(
                    parsed_btree_index.get(kv_item.0.clone()),
                    btree_index.get(kv_item.0.clone())
                );
            }
        }
    }

    // ... ... ... ... ... ... ... ... ... ... ... ... ... ... ... ..
}
