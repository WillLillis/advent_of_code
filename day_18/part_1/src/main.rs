// A quick sketch of how I'd like to solve this, just some initial thoughts
// 
// - There are 6 directions to examine the shape from to get its surface area,
// but because of symmetry, we only need to examine 3 of them
// - We can use a hash map for each viewing angle, storing the relevent 2
// coordinates and ignoring the third, as repeated inserts won't increment our count
//
// - The three coordinate pairs will be (x,z), (y,z), and (x,y)

fn main() {
    // TODO: parse input file into (x,y,z) tuples
    // implement the counting algo defined above
}
