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
            None => Err(index_errors::index_trait_remove_key_not_found()),
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
            None => Err(index_errors::index_trait_remove_key_not_found()),
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
                    std::collections::btree_map::Entry::Vacant(entry) => {
                        entry.insert(value.clone());
                        let tuple = kv::tuple::KVTuple::new_insert(&key, &value);
                        return Ok(tuple.to_bytes());
                    }
                    std::collections::btree_map::Entry::Occupied(entry) => {
                        return Err(index_errors::unique_index_trait_set_key_occupied());
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
            None => Err(index_errors::index_trait_remove_key_not_found()),
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
            None => Err(index_errors::index_trait_remove_key_not_found()),
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
}
