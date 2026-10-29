use crate::{ Device, Element, Model, Plan };


#[derive( Clone, Copy )]
pub struct Machine<'a>
{
	pub elements : &'a [ Element ],
	pub devices : &'a [ Device ],
	pub plan : &'a Plan,
	pub models : &'a [ Option<Model> ],
}
