use std::sync::Arc;
use tokio_cron_scheduler::{JobScheduler, Job};
use crate::service::file_service::FileService;

pub async fn init_delete_job(file_service: Arc<dyn FileService>) -> JobScheduler {
    let sched = JobScheduler::new()
        .await
        .expect("Failed to create Delete JobScheduler");

    let service_clone = file_service.clone();

    let cleanup_job = Job::new_async("0 0 3 * * *", move |_uuid, _l| {
        let service = service_clone.clone();
        Box::pin(async move {
            tracing::info!("🕒 [Cron] Starting daily file cleanup...");

            match service.cleanup_expired_files().await {
                Ok(_) => tracing::info!("✅ [Cron] Daily cleanup completed successfully."),
                Err(e) => tracing::error!("❌ [Cron] Daily cleanup failed: {:?}", e),
            }
        })
    })
        .expect("Failed to create cleanup job");

    sched.add(cleanup_job).await.expect("Failed to add cleanup job");

    sched.start().await.expect("Failed to start scheduler");

    tracing::info!("⏰ Cron Scheduler started successfully");

    sched
}