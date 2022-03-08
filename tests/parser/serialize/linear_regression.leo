/*
namespace: Serialize
expectation: Pass
*/

circuit Point {
    x: i32,
    y: i32,
    
    function new(x: i32, y: i32) -> Self { 
        return Self { x, y };
    }
}

circuit LinearRegression {
    points: [Point; 5],
    
    // Instantiates a linear regression circuit.
    function new(points: [Point; 5]) -> Self { 
        return Self { points };
    }
    
    // Return the slope of the linear regression.
    function slope(self) -> i32 { 
        
        let num_points: i32 = 5i32;
        // Calculate the sums.
        let x_sum: i32 = 0i32; 
        let y_sum: i32 = 0i32; 
        let xy_sum: i32 = 0i32; 
        let x2_sum: i32 = 0i32; 
        for i: u32 in 0u32..5u32 {
            x_sum += self.points[i].x;
            y_sum += self.points[i].y;
            xy_sum += self.points[i].x * self.points[i].y;
            x2_sum += self.points[i].x * self.points[i].x;
        }
        let numerator: i32 = (num_points * xy_sum) - (x_sum * y_sum); 
        let denominator: i32 = (num_points * x2_sum) - (x_sum * x_sum);
        let slope: i32 = numerator / denominator;
        return slope;
    }
    // Return the offset of the linear regression.
    function offset(self, slope: i32) -> i32 {
        let num_points: i32 = 5i32; 
        // Calculate the sum. 
        let x_sum: i32 = 0i32;
        let y_sum: i32 = 0i32;
        for i: u32 in 0u32..5u32 {
            x_sum += self.points[i].x;
            y_sum += self.points[i].y; 
        }
        return (y_sum - slope * x_sum) / num_points;
    }
}


function main (x: i32, y: i32) -> [i32; 2] {
  let points: [Point; 5] = [
      Point{x: x + 1i32, y: y + 1i32},
      Point{x: x + 2i32, y: y + 2i32},
      Point{x: x + 3i32, y: y + 3i32},
      Point{x: x + 4i32, y: y + 4i32},
      Point{x: x + 5i32, y: y + 5i32}
  ];
  let reg: LinearRegression = LinearRegression::new(points);
  let slope: i32 = reg.slope();
  let offset: i32 = reg.offset(slope);
  return [slope, offset];
}
