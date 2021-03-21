use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::mem;

pub struct SortedEntry<K, V>(pub K, pub V);

// impl <K, V> Default for SortedEntry<K, V> where K: Default, V: Default {
// 	fn default() -> Self {
// 		Self(Default::default(), Default::default())
// 	}
// }

// Is there a better way to do this? Like a derive that can fail without causing compiler errors?
impl <K, V> Clone for SortedEntry<K, V> where K: Clone, V: Clone {
	fn clone(&self) -> Self {
		Self(self.0.clone(), self.1.clone())
	}
}

impl <K, V> Copy for SortedEntry<K, V> where K: Copy, V: Copy {}

impl <K, V> Debug for SortedEntry<K, V> where K: Debug, V: Debug {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("SortedEntry")
			.field(&self.0)
			.field(&self.1)
			.finish()
	}
}

fn get_key<K: Ord+Clone, V>(entry: &SortedEntry<K, V>) -> K {
	entry.0.clone()
}

// Maybe remove these impl for ordering as they aren't used...
impl <K, V> Ord for SortedEntry<K, V> where K: Ord {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}

impl <K, V> PartialOrd for SortedEntry<K, V> where K: Ord {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.0.cmp(&other.0))
	}
}

impl <K, V> PartialEq for SortedEntry<K, V> where K: Ord {
    fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
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
			Ok(idx) => Some(mem::replace(&mut self[idx].1, val.clone())),
			Err(idx) => {
				self.insert(idx, SortedEntry(key.clone(), val.clone()));
				None
			},
		}
	}

	fn sorted_get_or_add(&mut self, key: K, val: V) -> &V {
		match self.sorted_searh(&key) {
			Ok(idx) => &self[idx].1,
			Err(idx) => {
				self.insert(idx, SortedEntry(key.clone(), val.clone()));
				&self[idx].1
			},
		}
	}

	fn sorted_get(&self, key: K) -> Option<&V> {
		match self.sorted_searh(&key) {
			Ok(idx) => Some(&self[idx].1),
			_ => None,
		}
	}

	fn sorted_remove(&mut self, key: K) -> Option<V> {
		match self.sorted_searh(&key) {
			Ok(idx) => Some(self.remove(idx).1),
			_ => None,
		}
	}

	fn sorted_searh(&self, key: &K) -> Self::SearchResult {
		self.binary_search_by_key(key, &get_key)
	}
}
