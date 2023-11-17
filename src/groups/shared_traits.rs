pub trait ToVec<T> {
    type VecSize;

    fn to_vec(&self) -> Self::VecSize;
}
