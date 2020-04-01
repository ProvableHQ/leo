// use snarkos_errors::gadgets::SynthesisError;
// use snarkos_models::{
//     curves::Field,
//     gadgets::{
//         r1cs::ConstraintSystem,
//         utilities::uint32::UInt32
//     }
// };
// use snarkos_models::gadgets::utilities::boolean::Boolean;
//
// impl UInt32 {
//     pub fn and<F: Field, CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, SynthesisError> {
//         let value= match (self.value, other.valoue) {
//             (Some(a), Some(b)) => Some(a & b),
//             _=> None,
//         };
//
//         let bits = self
//             .bits
//             .iter()
//             .zip(other.bits.iter())
//             .enumerate()
//             .map(|(i, (a, b)) | Boolean::and(cs.ns(|| format!("and of bit gadget {}", i)), a, b))
//             .collect();
//
//         Ok(UInt32 { bits, value })
//     }
//
//     fn recursive_add<F: Field, CS: ConstraintSystem<F>>(mut cs: CS, a: &Self, b: &Self) -> Result<Self, SynthesisError> {
//         let uncommon_bits = a.xor(cs.ns(|| format!("{} ^ {}", a.value.unwrap(), b.value.unwrap())),&b)?;
//         let common_bits = a.and(cs.ns(|| format!("{} & {}", a.value.unwrap(), b.value.unwrap())), &b)?;
//
//         if common_bits.value == 0 {
//             return Ok(uncommon_bits)
//         }
//         let shifted_common_bits = common_bits.rotr(common_bits.bits.len() - 1);
//         return Self::recursive_add(cs.ns(|| format!("recursive add {} + {}", uncommon_bits.value, shifted_common_bits.value)), &uncommon_bits, &shifted_common_bits)
//     }
//
//     pub fn add<F: Field, CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, SynthesisError> {
//         let new_value = match (self.value, other.value) {
//             (Some(a), Some(b)) => Some(a + b),
//             _ => None,
//         };
//
//         return Self::recursive_add(cs.ns( || format!("recursive add {} + {}", self.value, other.value)), &self, &other)
//
//         // let bits = self
//         //     .bits
//         //     .iter()
//         //     .zip(other.bits.iter())
//         //     .enumerate()
//         //     .map(|(i, (a, b))| Boo)
//     }
//
//     pub fn sub<F: Field, CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, SynthesisError> {}
//
//     pub fn mul<F: Field, CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, SynthesisError> {}
//
//     pub fn div<F: Field, CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, SynthesisError> {}
// }
