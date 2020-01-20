
struct A;

struct Single(A);

struct SingleGen<T>(T);

fn reg_fn(_s: Single) {}
fn gen_spec_t(_s: SingleGen<A>) {}
fn gen_spec_i32(_s: SingleGen<i32>) {}
fn generic<T>(_s: SingleGen<T>) {}

fn main() {
  // `Single` is finally concrete and explicitly takes `A`
  let s = Single(A);

  let _char: SingleGen<char> = SingleGen('a');

  let t       = SingleGen(A); // Uses `A` defined in file
  let my_i32  = SingleGen(6); // Uses `i32`
  let _char   = SingleGen('a'); // Uses `char`

  // Use function generics
  reg_fn(s);
  gen_spec_t(t);
  gen_spec_i32(my_i32);
  
  // explicitly specified type param `char` to `generic()`
  generic::<char>(SingleGen('a'));

  // Implicitly specified type parameter `char` to `generic()`
  generic(SingleGen('c'));
}
