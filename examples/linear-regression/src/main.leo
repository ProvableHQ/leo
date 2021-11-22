struct Point {
    x: i32,
    y: i32,
    
    function new(x: i32, y: i32) -> Self { 
        return Self { x, y };
    }
}

struct LinearRegression {
    points: [Point; 5],
    
    // Instantiates a linear regression struct.
    function new(points: [Point; 5]) -> Self { 
        return Self { points };
    }
    
    // Return the slope of the linear regression.
    function slope(self) -> i32 { 
        
        let num_points = 5i32;
        // Calculate the sums.
        let x_sum = 0i32; 
        let y_sum = 0i32; 
        let xy_sum = 0i32; 
        let x2_sum = 0i32; 
        for i in 0..5 {
            x_sum += self.points[i].x;
            y_sum += self.points[i].y;
            xy_sum += self.points[i].x * self.points[i].y;
            x2_sum += self.points[i].x * self.points[i].x;
        }
        let numerator = (num_points * xy_sum) - (x_sum * y_sum); 
        let denominator = (num_points * x2_sum) - (x_sum * x_sum);
        let slope = numerator / denominator;
        return slope;
    }
    // Return the offset of the linear regression.
    function offset(self, slope: i32) -> i32 {
        let num_points = 5i32; 
        // Calculate the sum. 
        let x_sum = 0i32;
        let y_sum = 0i32;
        for i in 0..5 {
            x_sum += self.points[i].x;
            y_sum += self.points[i].y; 
        }
        return (y_sum - slope * x_sum) / num_points;
    }
}


function main (x: i32, y: i32) -> [i32; 2] {
  let points: [Point; 5] = [
      Point{x: x + 1, y: y + 1},
      Point{x: x + 2, y: y + 2},
      Point{x: x + 3, y: y + 3},
      Point{x: x + 4, y: y + 4},
      Point{x: x + 5, y: y + 5}
  ];
  let reg = LinearRegression::new(points);
  let slope = reg.slope();
  let offset = reg.offset(slope);
  return [slope, offset];
}
