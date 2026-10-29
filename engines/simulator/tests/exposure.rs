use simulator::{ Element, Exposure, Kind, Net, Plan, Point, Segment };


const HIGH : f64 = 5.0;
const OHMS : f64 = 100.0;


fn led() -> Element
{
	Element { id : 1, kind : Kind::Led { color : 0 }, segment : Segment { from : Point { x : 0, y : 0 }, to : Point { x : 4, y : 0 } }, mirrored : false }
}

fn plan( lights : &[ ( usize, usize ) ], nets : usize ) -> Plan
{
	let mut plan = Plan { nets : vec![ Net::default(); nets ], lights : lights.iter().map( | &( anode, cathode ) | ( 0, anode, cathode ) ).collect(), ..Plan::default() };

	for ( light, &( anode, cathode ) ) in lights.iter().enumerate()
	{
		plan.nets[ anode ].lights.push( light );
		plan.nets[ cathode ].lights.push( light );
	}

	plan
}


#[test]
fn it_gives_the_mean_current_of_a_light_over_the_time_since_the_last_exposure()
{
	let plan = plan( &[ ( 0, 1 ) ], 2 );
	let elements = [ led() ];
	let mut levels = [ HIGH, 0.0 ];
	let mut exposure = Exposure::new( &plan, &elements, &levels, vec![ OHMS; 2 ], 0.0 );
	let lit = exposure.expose( 0.0 )[ 0 ];

	exposure.change( &plan, &levels, 0, 0.0, OHMS );
	levels[ 0 ] = 0.0;
	exposure.flush( &plan, &elements, &levels, 1.0 );

	assert!( lit > 1e-3 );
	assert!( ( exposure.expose( 4.0 )[ 0 ] - lit / 4.0 ).abs() < 1e-15 );
	assert_eq!( exposure.expose( 8.0 )[ 0 ], 0.0 );
}

#[test]
fn it_shares_the_drive_of_a_net_between_its_lit_lights()
{
	let alone = plan( &[ ( 0, 1 ) ], 2 );
	let shared = plan( &[ ( 0, 1 ), ( 0, 2 ) ], 3 );
	let elements = [ led() ];
	let single = Exposure::new( &alone, &elements, &[ HIGH, 0.0 ], vec![ OHMS; 2 ], 0.0 ).expose( 0.0 )[ 0 ];
	let mut levels = [ HIGH, 0.0, 0.0 ];
	let mut exposure = Exposure::new( &shared, &elements, &levels, vec![ OHMS; 3 ], 0.0 );
	let both = exposure.expose( 0.0 );

	exposure.change( &shared, &levels, 2, HIGH, OHMS );
	levels[ 2 ] = HIGH;
	exposure.flush( &shared, &elements, &levels, 0.0 );

	assert_eq!( both[ 0 ], both[ 1 ] );
	assert!( both[ 0 ] < single );
	assert_eq!( exposure.expose( 1.0 ), [ single, 0.0 ] );
}

#[test]
fn it_lights_a_light_once_when_both_of_its_nets_change_at_the_same_time()
{
	let plan = plan( &[ ( 0, 1 ) ], 2 );
	let elements = [ led() ];
	let lit = Exposure::new( &plan, &elements, &[ HIGH, 0.0 ], vec![ OHMS; 2 ], 0.0 ).expose( 0.0 )[ 0 ];
	let mut levels = [ 0.0, HIGH ];
	let mut exposure = Exposure::new( &plan, &elements, &levels, vec![ OHMS; 2 ], 0.0 );

	exposure.change( &plan, &levels, 0, HIGH, OHMS );
	levels[ 0 ] = HIGH;
	exposure.change( &plan, &levels, 1, 0.0, OHMS );
	levels[ 1 ] = 0.0;
	exposure.flush( &plan, &elements, &levels, 1.0 );

	assert!( ( exposure.expose( 2.0 )[ 0 ] - lit / 2.0 ).abs() < 1e-15 );
}

#[test]
fn it_follows_a_change_of_the_drive_of_a_net()
{
	let plan = plan( &[ ( 0, 1 ) ], 2 );
	let elements = [ led() ];
	let levels = [ HIGH, 0.0 ];
	let mut exposure = Exposure::new( &plan, &elements, &levels, vec![ OHMS; 2 ], 0.0 );
	let before = exposure.expose( 0.0 )[ 0 ];

	exposure.change( &plan, &levels, 0, HIGH, 10.0 * OHMS );
	exposure.flush( &plan, &elements, &levels, 0.0 );

	assert!( exposure.expose( 1.0 )[ 0 ] < before );
}
