use shared_type::{IntoShared, WithSharedInner};

fn main() {
    let vec = vec![1, 2, 3];
    let vec_shared = vec.into_shared(); // Arc<Mutex<Vec<i32>>>

    // The simple example of using the shared value that will wait for
    // the lock to become available and then call the closure
    vec_shared.with_inner(|vec| {
        vec.push(4);
    });

    // We can also use the `with` method, which returns an Option
    let result = vec_shared.with_inner(|vec| {
        vec.push(5);
        vec.len()
    });
    println!("vec_shared len: {}", result.unwrap());

    // Or we can use the `try_with` method, which will return None if the lock is not available
    let result = vec_shared.try_with_inner(|vec| {
        vec.push(6);
        vec.len()
    });
    println!("vec_shared len: {}", result.unwrap());
}
