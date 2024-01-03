#[allow(unused_imports)]
use morsk::{traits::Morskable, Word};

/* Does not compile. Error shows that Morskable<T> is a "sealed trait."
struct T {}
impl Morskable<10> for T {}
*/
