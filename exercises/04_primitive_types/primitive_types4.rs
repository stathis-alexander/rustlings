fn main() {
  // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
  #[test]
  fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
    // let nice_slice = ???
    // this is somewhat mysterious. my mental model for & comes from C, in which case this would be the address of the
    // pointer to the first element of the array. one could store this in a double pointer
    // ptr** -> ptr* -> value
    // but in Rust, somehow there is a deeper semantic meaning to this object.
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice);
  }
}
