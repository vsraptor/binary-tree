//based on the discussion here https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
// Binary tree implementation (simplified)

#![allow(dead_code)]

use std::string::String;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;

struct BinTree<T> {
	content : T,
	left : Option<Box<BinTree<T>>>,
	right : Option<Box<BinTree<T>>>
}

impl<T: PartialEq + PartialOrd> BinTree<T> {

	fn new(val: T) -> BinTree<T> {
		BinTree { content : val, left: None, right: None }
	} 

	fn add(&mut self, val: T) {
		if self.content == val {return};//already exists
		let update = if val > self.content { &mut self.right } else { &mut self.left };
		match update {
			Some(update) => update.add(val), //dig deeper
			None => *update = Some(Box::new(BinTree::new(val))) //add a leaf 
		}
	}

	fn search(&self, target: T) -> Option<T> {

		if target == self.content { Some(target) } //found
		else if target < self.content { self.left.as_ref()?.search(target) }
		else if target > self.content { self.right.as_ref()?.search(target) }
		else { None } //not found
	}
}

fn main() {
   let mut root = BinTree::new(5);
   root.add(7);
   root.add(3);

   println!("{:#?}", root );

   let mut s = BinTree::new(String::from("angle"));
   s.add(String::from("all"));
   s.add(String::from("bell"));


   println!("{:#?}", s );
   let res = s.search(String::from("bell"));
   println!("{:#?}", res );
}


