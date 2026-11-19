use crate::*;

pub fn signExtend<P: Platform>(ctx: &Context<P>, mut raw: f64, mut bits: f64) -> f64 {
let sign: f64 = numbers::shl(1.0f64, ((bits - 1.0f64)));
let range: f64 = numbers::shl(1.0f64, bits);
return (if ((numbers::and(raw, sign)) != 0.0f64) { (raw - range) } else { raw });
}


pub fn rv64ImmediateI<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> f64 {
return crate::m_vm__rv64_immediate::signExtend(ctx, numbers::ushr(instruction, 20.0f64), 12.0f64);
}


pub fn rv64ImmediateS<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> f64 {
return crate::m_vm__rv64_immediate::signExtend(ctx, numbers::or(numbers::shl((numbers::ushr(instruction, 25.0f64)), 5.0f64), (numbers::and(numbers::ushr(instruction, 7.0f64), 31.0f64))), 12.0f64);
}


pub fn rv64ImmediateB<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> f64 {
return crate::m_vm__rv64_immediate::signExtend(ctx, numbers::or(numbers::or(numbers::or(numbers::shl((numbers::ushr(instruction, 31.0f64)), 12.0f64), numbers::shl((numbers::and(numbers::ushr(instruction, 7.0f64), 1.0f64)), 11.0f64)), numbers::shl((numbers::and(numbers::ushr(instruction, 25.0f64), 63.0f64)), 5.0f64)), numbers::shl((numbers::and(numbers::ushr(instruction, 8.0f64), 15.0f64)), 1.0f64)), 13.0f64);
}


pub fn rv64ImmediateU<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> f64 {
return numbers::and(instruction, 4294963200.0f64);
}


pub fn rv64ImmediateJ<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> f64 {
return crate::m_vm__rv64_immediate::signExtend(ctx, numbers::or(numbers::or(numbers::or(numbers::shl((numbers::ushr(instruction, 31.0f64)), 20.0f64), numbers::shl((numbers::and(numbers::ushr(instruction, 12.0f64), 255.0f64)), 12.0f64)), numbers::shl((numbers::and(numbers::ushr(instruction, 20.0f64), 1.0f64)), 11.0f64)), numbers::shl((numbers::and(numbers::ushr(instruction, 21.0f64), 1023.0f64)), 1.0f64)), 21.0f64);
}


pub fn rv64CompressedJump<P: Platform>(ctx: &Context<P>, mut instruction: f64) -> f64 {
return crate::m_vm__rv64_immediate::signExtend(ctx, numbers::or(numbers::or(numbers::or(numbers::or(numbers::or(numbers::or(numbers::or((numbers::and(numbers::ushr(instruction, 1.0f64), 2048.0f64)), (numbers::and(numbers::shl(instruction, 2.0f64), 1024.0f64))), (numbers::and(numbers::ushr(instruction, 1.0f64), 768.0f64))), (numbers::and(numbers::shl(instruction, 1.0f64), 128.0f64))), (numbers::and(numbers::ushr(instruction, 1.0f64), 64.0f64))), (numbers::and(numbers::shl(instruction, 3.0f64), 32.0f64))), (numbers::and(numbers::ushr(instruction, 7.0f64), 16.0f64))), (numbers::and(numbers::ushr(instruction, 2.0f64), 14.0f64))), 12.0f64);
}

