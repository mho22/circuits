use crate::{ Element, Kind, Point, Segment, Shape };


#[derive( Clone, Debug, PartialEq )]
pub enum Operation
{
	Add { kind : Kind, segment : Segment, mirrored : bool },
	Insert { element : Element, index : usize },
	Delete { id : u32 },
	SetValue { id : u32, value : f64 },
	SetClosed { id : u32, closed : bool },
	SetHigh { id : u32, high : bool },
	Move { id : u32, kind : Kind, segment : Segment, mirrored : bool },
	Describe { name : String, drawing : Vec<Shape>, layout : Vec<Point>, delay : f64 },
	Label { id : u32, label : String },
}
