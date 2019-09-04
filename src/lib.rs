use std::cmp::Ordering;
use std::mem;

pub struct SortedEntry<K, V> {
	pub key: K,
	pub val: V,
}

fn get_key<K: Ord+Clone, V>(entry: &SortedEntry<K, V>) -> K {
	entry.key.clone()
}

// Maybe remove these impl for ordering as they aren't used...
impl <K, V> Ord for SortedEntry<K, V> where K: Ord {
	fn cmp(&self, other: &Self) -> Ordering {
		self.key.cmp(&other.key)
	}
}

impl <K, V> PartialOrd for SortedEntry<K, V> where K: Ord {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.key.cmp(&other.key))
	}
}

impl <K, V> PartialEq for SortedEntry<K, V> where K: Ord {
    fn eq(&self, other: &Self) -> bool {
		self.key == other.key
	}
}

impl <K, V> Eq for SortedEntry<K, V> where K: Ord {}

pub trait SortedCollection<K, V> {
	type SearchResult;
	// map like insert, i.e. overrides current value
	fn sorted_insert(&mut self, key: K, val: V) -> Option<V>;
	fn sorted_get_or_add(&mut self, key: K, val: V) -> &V;
	fn sorted_get(&self, key: K) -> Option<&V>;
	fn sorted_remove(&mut self, key: K) -> Option<V>;
	fn sorted_searh(&self, key: &K) -> Self::SearchResult;
}

impl <K: Ord + Clone, V: Clone> SortedCollection<K, V> for Vec<SortedEntry<K, V>> {
	type SearchResult = Result<usize, usize>;

	fn sorted_insert(&mut self, key: K, val: V) -> Option<V> {
		match self.sorted_searh(&key) {
			Ok(idx) => Some(mem::replace(&mut self[idx].val, val.clone())),
			Err(idx) => {
				self.insert(idx, SortedEntry{key: key.clone(), val: val.clone()});
				None
			},
		}
	}

	fn sorted_get_or_add(&mut self, key: K, val: V) -> &V {
		match self.sorted_searh(&key) {
			Ok(idx) => &self[idx].val,
			Err(idx) => {
				self.insert(idx, SortedEntry{key: key.clone(), val: val.clone()});
				&self[idx].val
			},
		}
	}

	fn sorted_get(&self, key: K) -> Option<&V> {
		match self.sorted_searh(&key) {
			Ok(idx) => Some(&self[idx].val),
			_ => None,
		}
	}

	fn sorted_remove(&mut self, key: K) -> Option<V> {
		match self.sorted_searh(&key) {
			Ok(idx) => Some(self.remove(idx).val),
			_ => None,
		}
	}

	fn sorted_searh(&self, key: &K) -> Self::SearchResult {
		self.binary_search_by_key(key, &get_key)
	}
}