use simulator::{ Drive, THRESHOLD };


#[test]
fn it_reads_a_drive_above_the_threshold_as_high()
{
	assert!( Drive { volts : 5.0, ohms : 10.0 }.high() );
	assert!( ! Drive { volts : THRESHOLD, ohms : 10.0 }.high() );
	assert!( ! Drive { volts : 0.0, ohms : 10.0 }.high() );
}
