/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

use std::ops::Deref;
#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	fn len(&self) -> usize {
		self.size
	}
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	fn pop(&mut self) -> Option<T> {
		// TODO
		if self.is_empty(){
			None
		}else {
			// vec的pop可以直接返回最后一个元素
			self.size-=1;
			self.data.pop()
		}		
		
	}
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;self.0.data.pop()
		} 
		else {
			None
		}
	}
}
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}

fn bracket_match(bracket: &str) -> bool
{
	if bracket.len() < 1{
		true
	}else {
		let mut stack_bra:Stack<char> = Stack::new();
		let mut s:Vec<char> = bracket.chars().collect();
		
		stack_bra.push(s[0]);
		let mut count = 1;
		let leng = s.len();
		// 当下一个括号是左括号，入栈
		// 当下一个括号是右括号，将栈顶弹出进行对比，如果对得上就下一个，对不上就false
		// 如果还有其他特殊情况，一律为false
		
		while !stack_bra.is_empty() && count < leng {
			
			match s[count] {
				
				left if left == '(' || left == '[' || left == '{' =>{
					stack_bra.push(left);
				},
				right1 if right1 == ')' => {
					if stack_bra.pop().unwrap() != '('{
						return false;
					}
				},
				right2 if right2 == ']' => {
					if stack_bra.pop().unwrap() != '['{
						return false;
					}
				},
				right3 if right3 == '}' => {
					if stack_bra.pop().unwrap() != '{'{
						return false;
					}
				},
				_ => {},
			}
			count+=1;
			// 如果因为匹配弹出了一个左括号而导致栈空，压入一个左括号
			if stack_bra.is_empty() && count < leng{
				match s[count] {
					left if left == '(' || left == '[' || left == '{' =>{
						stack_bra.push(left);
						count+=1;
					},
					_=> return false,
				}
			}
		}
		true
	}
	
}





#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}