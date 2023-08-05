struct TriangleCoordinate{
    time_position: usize,
    space_position: usize,
}

impl TriangleCoordinate{
    fn new(time_position: usize, space_position: usize) -> Self{
        Self{
            time_position,
            space_position,
        }
    }
}

struct NodeCoordinate{
    time_position: usize,
    space_position: usize,
}

impl NodeCoordinate{
    fn new(time_position: usize, space_position: usize) -> Self{
        Self{
            time_position,
            space_position,
        }
    }
}