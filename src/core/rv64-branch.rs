use crate::*;

pub fn rv64Branch<P: Platform>(ctx: &Context<P>, mut funct3: f64, mut left: i128, mut right: i128) -> f64 {
if (funct3 == 0.0f64) {
return (if (left == right) { 1.0f64 } else { 0.0f64 });
}
if (funct3 == 1.0f64) {
return (if (left != right) { 1.0f64 } else { 0.0f64 });
}
if (funct3 == 4.0f64) {
return (if (left < right) { 1.0f64 } else { 0.0f64 });
}
if (funct3 == 5.0f64) {
return (if (left >= right) { 1.0f64 } else { 0.0f64 });
}
if ((funct3 == 6.0f64) || (funct3 == 7.0f64)) {
{
let leftNegative: bool = (left < 0i128);
let rightNegative: bool = (right < 0i128);
let less: bool = (if (leftNegative == rightNegative) { (left < right) } else { (!leftNegative) });
return (if ((if (funct3 == 6.0f64) { less } else { (!less) })) { 1.0f64 } else { 0.0f64 });
}
}
return (-1.0f64);
}

