use super::*;

impl Line {
	#[inline]
	pub fn x1(self, x1: impl num_traits::Num + std::string::ToString) -> Self {
		self.set_x1(x1);
		self
	}

	#[inline]
	pub fn set_x1(&self, x1: impl num_traits::Num + std::string::ToString) {
		self.set_attr(web_str::x1(), x1.to_string());
	}

	#[inline]
	pub fn x2(self, x2: impl num_traits::Num + std::string::ToString) -> Self {
		self.set_x2(x2);
		self
	}

	#[inline]
	pub fn set_x2(&self, x2: impl num_traits::Num + std::string::ToString) {
		self.set_attr(web_str::x2(), x2.to_string());
	}

	#[inline]
	pub fn y1(self, y1: impl num_traits::Num + std::string::ToString) -> Self {
		self.set_y1(y1);
		self
	}

	#[inline]
	pub fn set_y1(&self, y1: impl num_traits::Num + std::string::ToString) {
		self.set_attr(web_str::y1(), y1.to_string());
	}

	#[inline]
	pub fn y2(self, y2: impl num_traits::Num + std::string::ToString) -> Self {
		self.set_y2(y2);
		self
	}

	#[inline]
	pub fn set_y2(&self, y2: impl num_traits::Num + std::string::ToString) {
		self.set_attr(web_str::y2(), y2.to_string());
	}

	#[inline]
	pub fn path_length(self, path_length: impl num_traits::Num + std::string::ToString) -> Self {
		self.set_path_length(path_length);
		self
	}

	#[inline]
	pub fn set_path_length(&self, path_length: impl num_traits::Num + std::string::ToString) {
		self.set_attr(web_str::path_length(), path_length.to_string());
	}
}

impl Svg {
	#[inline]
	pub fn viewbox(
		self,
		min_x: impl num_traits::Num + std::string::ToString,
		min_y: impl num_traits::Num + std::string::ToString,
		width: impl num_traits::Num + std::string::ToString,
		height: impl num_traits::Num + std::string::ToString,
	) -> Self {
		self.set_viewbox(min_x, min_y, width, height);
		self
	}

	#[inline]
	pub fn set_viewbox(
		&self,
		min_x: impl num_traits::Num + std::string::ToString,
		min_y: impl num_traits::Num + std::string::ToString,
		width: impl num_traits::Num + std::string::ToString,
		height: impl num_traits::Num + std::string::ToString,
	) {
		self.attr(web_str::viewBox(), format!("{} {} {} {}", min_x.to_string(), min_y.to_string(), width.to_string(), height.to_string()));
	}
}
