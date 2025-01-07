use future03::compat::Future01CompatExt;
use futures_cpupool::Builder;
mod ten_polls;

fn main() {
    let pool = Builder::new().pool_size(4).name_prefix("my-pool-").create();
    let f01 = pool.spawn_fn(move || {
        let f = ten_polls::TenPolls::new("tom".to_string());
        Box::new(f)
    });
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    let r = runtime.block_on(async move {
        let r = f01.compat().await.unwrap();
        r
    });
    println!("result: {:?}", r);
}
