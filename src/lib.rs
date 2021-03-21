use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::mem;

pub struct SortedEntry<K, V> {
	pub key: K,
	pub val: V,
}

// Is there a better way to do this? Like a derive that can fail without causing compiler errors?
impl <K, V> Default for SortedEntry<K, V> where K: Default, V: Default {
	fn default() -> Self {
		Self{key: Default::default(), val: Default::default()}
	}
}

impl <K, V> Clone for SortedEntry<K, V> where K: Clone, V: Clone {
	fn clone(&self) -> Self {
		Self{key: self.key.clone(), val: self.val.clone()}
	}
}

impl <K, V> Copy for SortedEntry<K, V> where K: Copy, V: Copy {}

impl <K, V> Debug for SortedEntry<K, V> where K: Debug, V: Debug {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SortedEntry")
			.field("key" ,&self.key)
			.field("val", &self.val)
			.finish()
	}
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

pub type SortedVecMap<K, V> = Vec<SortedEntry<K, V>>;

impl <K: Ord + Clone, V: Clone> SortedCollection<K, V> for SortedVecMap<K, V> {
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
