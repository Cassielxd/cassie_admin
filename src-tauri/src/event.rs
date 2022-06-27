use async_std::prelude::StreamExt;
use pharos::{Channel, SharedPharos};
use std::thread;

use crate::APPLICATION_CONTEXT;
#[derive(Clone, Debug, PartialEq)]
pub enum CassieEvent {}

//初始化 event bus事件处理器
pub fn init_event_bus() {
    APPLICATION_CONTEXT.set::<SharedPharos<CassieEvent>>(SharedPharos::default());
    //这里只能使用thread::spawn
    thread::spawn(|| {
        tauri::async_runtime::block_on(async { init_consumer().await });
    });
}

//事件消费处理类
pub async fn init_consumer() {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    let mut events = pharos.observe_shared(Channel::Unbounded.into()).await.unwrap();

    loop {
        let event = events.next().await.unwrap();
        consume(event).await;
    }
}

//事件消费
pub async fn consume(event: CassieEvent) {
    match event {}
}

//发布事件
pub async fn fire_event(e: CassieEvent) {
    tauri::async_runtime::block_on(async {
        let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
        let _r = pharos.notify(e).await;
    });
}
