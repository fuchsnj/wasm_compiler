use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;

pub struct Inner<S: IndexSection>{
	sections: HashMap<S, Vec<Handle>>
}
impl<S> Inner<S> where S: IndexSection{
	pub fn new() -> Inner<S>{
		Inner{
			sections: HashMap::new()
		}
	}
	pub fn get_index(&self, index_section: S, index: u64) -> u64{
		let mut total = 0;
		for section in S::get_sections(){
			if section == index_section{
				return total + index;
			}else{
				total += self.size_of_section(&section) ;
			}
		}
		unreachable!();
	}
	fn size_of_section(&self, section: &S) -> u64{
		if let Some(section) = self.sections.get(section){
			section.len() as u64
		}else{
			0
		}
	}
}

pub struct IndexSpace<S: IndexSection>{
	inner: Arc<Mutex<Inner<S>>>
}
impl<S> IndexSpace<S> where S: IndexSection{
	pub fn new() -> IndexSpace<S>{
		IndexSpace{
			inner: Arc::new(Mutex::new(Inner::new()))
		}
	}
	pub fn allocate_index(&mut self, section: S) -> Handle{
		let section_clone = self.inner.clone();
		let mut inner = self.inner.lock().unwrap();
		let entry = inner.sections.entry(section).or_insert_with(||{vec!()});
		let current_section_index = entry.len() as u64;
		let handle = Handle::new(move ||{
			section_clone.lock().unwrap().get_index(section, current_section_index)
		});
		entry.push(handle.clone());
		handle
		/*
				let sections = S::get_sections();
		self.inner.lock().unwrap().get_index(sections, self.index_section, self.index)
		 */
	}
}

pub trait IndexSection where Self: Clone + Copy + Eq + Hash + PartialEq + 'static{
	fn get_sections() -> Vec<Self>;
}

#[derive(Clone)]
pub struct Handle{
	func: Rc<Fn() -> u64>
}
impl Handle{
	pub fn new<F>(get_index: F) -> Handle
	where F: Fn() -> u64 + 'static{
		Handle{
			func: Rc::new(get_index)
		}
	}
	pub fn get_index(&self) -> u64{
		(self.func)()
	}
}