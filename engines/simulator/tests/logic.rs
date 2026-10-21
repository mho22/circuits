use simulator::{ Action, Logic };


const FOO : Action = Action::Fire { device : 0, stamp : 1 };
const BAR : Action = Action::Fire { device : 1, stamp : 1 };
const BAZ : Action = Action::Edge { clock : 0, count : 1 };


#[test]
fn it_gives_the_events_up_to_a_time_earliest_first_and_in_order_of_scheduling()
{
	let mut logic = Logic::default();

	logic.schedule( 2e-9, FOO );
	logic.schedule( 1e-9, BAR );
	logic.schedule( 2e-9, BAZ );

	assert!( logic.pending( 1e-9 ) );
	assert_eq!( logic.next( 1e-9 ).map( | event | event.action ), Some( BAR ) );
	assert_eq!( logic.next( 1e-9 ), None );
	assert!( ! logic.pending( 1e-9 ) );
	assert_eq!( [ logic.next( 2e-9 ), logic.next( 2e-9 ) ].map( | event | event.map( | event | event.action ) ), [ Some( FOO ), Some( BAZ ) ] );
}

#[test]
fn it_drops_every_event_on_a_clear()
{
	let mut logic = Logic::default();

	logic.schedule( 0.0, FOO );
	logic.clear();

	assert!( ! logic.pending( 1.0 ) );
}
