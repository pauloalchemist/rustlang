mod front_of_house {
	pub mod hostings {
  	pub fn add_to_waitlist() {}

  }
}

pub fn eat_at_restaurant() {
	// absolute path
	crate::front_of_house::hostings::add_to_waitlist();
	// relative path
	front_of_house::hostings::add_to_waitlist();

}

fn server_order() {}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::server_order();
	}

	fn cook_order() {}
}