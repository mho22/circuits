use simulator::{ Action, Event };


fn event( time : f64, sequence : u64 ) -> Event
{
	Event { time, sequence, action : Action::Fire { device : 0, stamp : 0 } }
}


#[test]
fn it_orders_events_by_time_then_by_sequence()
{
	assert!( event( 1e-9, 2 ) < event( 2e-9, 1 ) );
	assert!( event( 1e-9, 1 ) < event( 1e-9, 2 ) );
	assert_eq!( event( 1e-9, 1 ), Event { action : Action::Edge { clock : 0, count : 1 }, ..event( 1e-9, 1 ) } );
}
