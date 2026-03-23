
struct Point { x: u32, y: u32 }

struct Vec::[N: u32] { data: u32 }

struct Matrix::[R: u32, C: u32] { rows: u32, cols: u32 }
const SIZE: u32 = 8u32;
struct Grid::[N: u32] { cells: u32 }
struct Board { grid: Grid::[SIZE] }
