use std::{ slice, mem };

pub unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
  let data_ptr: *const u8 = mem::transmute(s);
  let slice = slice::from_raw_parts(data_ptr, mem::size_of::<T>());
  slice
}

pub unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
  let data_ptr: *const u8 = mem::transmute(&s[0]);
  let slice = slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len());
  slice
}
