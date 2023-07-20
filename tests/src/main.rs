use reexport_impl::reexport;

pub struct Inner {
    n: usize,
}

#[reexport(Outer::inner)]
impl Inner {
    pub fn describe_inner(&self, k: usize) {
        println!("Inner: n = {}, k = {}", self.n, k);
    }
}

pub struct Outer {
    inner: Inner,
}

fn main() {
    let o = Outer {
        inner: Inner { n: 0 },
    };
    o.describe_inner(2);
}
