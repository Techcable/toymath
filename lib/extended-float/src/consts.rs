use extended_float_macros::extended_float;

use super::ExtendedFloat;

pub const NAN: ExtendedFloat = extended_float!("NAN");
pub const INFINITY: ExtendedFloat = extended_float!("inf");
pub const NEG_INFINITY: ExtendedFloat = extended_float!("-inf");
pub const PI: ExtendedFloat = extended_float!("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862");

// Pi fractions
/// The value of `pi/2`
pub const FRAC_PI_2: ExtendedFloat = extended_float!("1.5707963267948966192313216916397514420985846996875529104874722961539082");
/// The value of `pi/3`
pub const FRAC_PI_3: ExtendedFloat = extended_float!("1.0471975511965977461542144610931676280657231331250352736583148641026054");
/// The value of `pi/4`
pub const FRAC_PI_4: ExtendedFloat = extended_float!("0.7853981633974483096156608458198757210492923498437764552437361480769541");
/// The value of `pi/6`
pub const FRAC_PI_6: ExtendedFloat = extended_float!("0.5235987755982988730771072305465838140328615665625176368291574320513027");

// Sqrt constants
/// the value of `1/sqrt(2)`
pub const FRAC_1_SQRT_2: ExtendedFloat = extended_float!("0.7071067811865475244008443621048490392848359376884740365883398689953662");
