use std::{
	borrow::Borrow,
	hash::Hash
};
use serde_json::{
	Value,
	Map
};
use crate::{
	Collection,
	Len,
	Get,
	GetMut,
	MapInsert,
	Remove,
	Clear
};

impl Collection for Map<String, Value> {
	type Item = Value;
}

impl Len for Map<String, Value> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.len()
	}

	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl <Q: ?Sized> Get<&Q> for Map<String, Value> where String: Borrow<Q>, Q: Hash + Eq + Ord {
	#[inline(always)]
	fn get<'a>(&'a self, key: &Q) -> Option<&'a Value> {
		self.get(key)
	}
}

impl<Q: ?Sized> GetMut<&Q> for Map<String, Value> where String: Borrow<Q>, Q: Hash + Eq + Ord {
	#[inline(always)]
	fn get_mut<'a>(&'a mut self, key: &Q) -> Option<&'a mut Value> {
		self.get_mut(key)
	}
}

impl MapInsert<String> for Map<String, Value> {
	type Output = Option<Value>;

	#[inline(always)]
	fn insert(&mut self, key: String, value: Value) -> Option<Value> {
		self.insert(key, value)
	}
}

impl<Q: ?Sized> Remove<&Q> for Map<String, Value> where String: Borrow<Q>, Q: Hash + Eq + Ord {
	#[inline(always)]
	fn remove(&mut self, key: &Q) -> Option<Value> {
		self.remove(key)
	}
}

impl Clear for Map<String, Value> {
	#[inline(always)]
	fn clear(&mut self) {
		self.clear()
	}
}