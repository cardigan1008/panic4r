// src/main.rs
use tokio::runtime::Runtime;
use tokio::task::LocalSet;

thread_local! {
    pub static LOCAL_SET: TokioLocalSet = TokioLocalSet::new();
}

// the order here seems wrong, 
// have revised, see the following comments
pub struct TokioLocalSet {
    rt: Runtime,
    local: LocalSet,
}

impl TokioLocalSet {
    pub fn new() -> TokioLocalSet {
        TokioLocalSet {
            rt: tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
            local: LocalSet::new(),
        }
    }

    async fn inner_method(&self) {
        self.local
            .run_until(async move {
                tokio::task::spawn_local(async {});
            })
            .await
    }

    pub fn method(&self) {
        self.rt.block_on(self.inner_method());
    }
}

fn main(){
    // will panic
    LOCAL_SET.with(|f|{
        f.method();
    });
}


