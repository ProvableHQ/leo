// use snarkos_utilities::serialize::*;
// use snarkos_algorithms::snark::groth16::KeypairAssembly;
//
//
// pub struct CircuitBytes<E> {
//     assembly: KeypairAssembly<E>
// }

// impl<E: > CanonicalSerialize for CircuitBytes<E> {

// }

// impl CircuitBytes {
// use snarkos_utilities::serialize::*;
// fn serialize_constraints<E: PairingEngine>(assembly: KeypairAssembly<E>) {
//     let mut writer = Vec::new();
//     // println!("{}", assembly.num_constraints);
//
//     let fr_size = <snarkos_curves::edwards_bls12::Fq as ConstantSerializedSize>::SERIALIZED_SIZE;
//     let index_size = <snarkos_models::gadgets::r1cs::Index as ConstantSerializedSize>::SERIALIZED_SIZE;
//     let tuple_size = fr_size + index_size;
//     println!("tuple size {}", tuple_size);
//
//     let mut total_size_bytes = 0;
//     // serialize each constraint
//     for i in 0..assembly.num_constraints {
//         // serialize each linear combination
//         // println!("a len {}", a_len);
//         // println!("b len {}", assembly.bt[i].len());
//         // println!("c len {}", assembly.ct[i].len());
//
//         // Serialize the at[i] vector of tuples Vec<(Fr, Index)>
//
//         let a_len = assembly.at[i].len();
//         CanonicalSerialize::serialize(&(a_len as u8), &mut writer).unwrap();
//         total_size_bytes += 1;
//
//         total_size_bytes += a_len * tuple_size;
//
//         for &(ref coeff, index) in assembly.at[i].iter() {
//             // println!("a({:?}, {:?})", coeff, index);
//             CanonicalSerialize::serialize(coeff, &mut writer).unwrap();
//             CanonicalSerialize::serialize(&index, &mut writer).unwrap();
//         }
//
//         // Serialize the bt[i] vector of tuples Vec<(Fr, Index)>
//
//         let b_len = assembly.bt[i].len();
//         CanonicalSerialize::serialize(&(b_len as u8), &mut writer).unwrap();
//         total_size_bytes += 1;
//
//         total_size_bytes += b_len * tuple_size;
//
//         for &(ref coeff, index) in assembly.bt[i].iter() {
//             // println!("b({:?}, {:?})", coeff, index);
//             CanonicalSerialize::serialize(coeff, &mut writer).unwrap();
//             CanonicalSerialize::serialize(&index, &mut writer).unwrap();
//         }
//
//         // Serialize the ct[i] vector of tuples Vec<(Fr, Index)>
//
//         let c_len = assembly.ct[i].len();
//         CanonicalSerialize::serialize(&(c_len as u8), &mut writer).unwrap();
//         total_size_bytes += 1;
//
//         total_size_bytes += c_len * tuple_size;
//
//         for &(ref coeff, index) in assembly.ct[i].iter() {
//             // println!("c({:?}, {:?})", coeff, index);
//             CanonicalSerialize::serialize(coeff, &mut writer).unwrap();
//             CanonicalSerialize::serialize(&index, &mut writer).unwrap();
//         }
//     }
//     println!("expected size bytes {:?}", total_size_bytes);
//     println!("actual size bytes {:?}", writer.len());
// }
//
//
// let mut cs = KeypairAssembly::<Bls12_377> {
// num_inputs: 0,
// num_aux: 0,
// num_constraints: 0,
// at: vec![],
// bt: vec![],
// ct: vec![],
// };
// let temporary_program = program.clone();
// let output = temporary_program.compile_constraints(&mut cs)?;
// log::debug!("Compiled constraints - {:#?}", output);
// log::debug!("Number of constraints - {:#?}", cs.num_constraints());
//
// serialize_constraints(cs);
//
// // println!("{}", std::mem::size_of::<snarkos_curves::bls12_377::Fq>());
// // println!("{}", std::mem::size_of::<snarkos_curves::edwards_bls12::Fq>());
// // println!("{}", <snarkos_models::gadgets::r1cs::Index as ConstantSerializedSize>::SERIALIZED_SIZE);
// // println!("{}", <snarkos_curves::edwards_bls12::Fq as ConstantSerializedSize>::SERIALIZED_SIZE);
//
// // println!("{}", std::mem::size_of::<snarkos_models::gadgets::r1cs::Index>());
// }
