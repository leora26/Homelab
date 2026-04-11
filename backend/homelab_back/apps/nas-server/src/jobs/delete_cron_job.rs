use crate::service::file_service::FileService;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::service::clean_up_service::CleanUpService;

pub async fn init_delete_job(clean_up_service: Arc<dyn CleanUpService>) -> JobScheduler {
    let sched = JobScheduler::new()
        .await
        .expect("Failed to create Delete JobScheduler");

    let service_clone = clean_up_service.clone();

    let cleanup_job = Job::new_async("0 0 3 * * *", move |_uuid, _l| {
        let service = service_clone.clone();
        Box::pin(async move {
            tracing::info!("🕒 [Cron] Starting daily file cleanup...");

            match service.hard_delete_all_trash().await {
                Ok(_) => tracing::info!("✅ [Cron] Daily cleanup completed successfully."),
                Err(e) => tracing::error!("❌ [Cron] Daily cleanup failed: {:?}", e),
            }
        })
    })
    .expect("Failed to create cleanup job");

    sched
        .add(cleanup_job)
        .await
        .expect("Failed to add cleanup job");

    sched.start().await.expect("Failed to start scheduler");

    tracing::info!("⏰ Cron Scheduler started successfully");

    sched
}
