use simulator::Error;


#[test]
fn it_describes_each_error()
{
	assert_eq!( Error::Crooked.to_string(), "A wire must run at 0°, 45° or 90°." );
	assert_eq!( Error::Empty.to_string(), "A wire must have a length." );
	assert_eq!( Error::Missing.to_string(), "No wire has this id." );
}
