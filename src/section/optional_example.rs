// rustにはnullがない
// 例外機構もない

// 値が存在しない時はNode
// 値が存在するときはSome(T)
enum Optional<T> {
    None,
    Some(T),
}
