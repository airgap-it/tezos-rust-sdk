pub trait InnerValueRef<T> {
    fn inner_value_ref(&self) -> Option<&T>;
}
