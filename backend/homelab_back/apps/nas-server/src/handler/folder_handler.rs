use crate::AppState;
use actix_web::{delete, get, patch, web, HttpResponse, Responder};
use actix_web::web::{Data, Json, Path, Query};
use uuid::Uuid;
use crate::data::delete_chosen_folders_command::DeleteChosenFoldersCommand;
use crate::data::filter_files_by_filetype_command::FilterFilesByFileTypeCommand;
use crate::data::search_query::SearchQuery;
use crate::data::update_folder_name_command::UpdateFolderNameCommand;
use crate::helpers::error_mapping::map_data_err_to_http;

#[get("/folders/{userId}/root")]
pub async fn get_root_folder(
    app_state: Data<AppState>,
    id: Path<Uuid>,
) -> impl Responder {

    let user_id = id.into_inner();

    match app_state.folder_service.get_root(user_id).await {
        Ok(Some(folder)) => HttpResponse::Ok().json(folder),
        Ok(None) => HttpResponse::NotFound().body(format!(
            "No root folder was found for user with id: {}",
            user_id
        )),
        Err(e) => {
            tracing::error!("Failed to fetch root folder: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}

#[get("/folders/{folderId}")]
pub async fn get_folder_by_id(
    app_state: Data<AppState>,
    id: Path<Uuid>,
) -> impl Responder {

    let folder_id = id.into_inner();

    match app_state.folder_service.get_by_id(folder_id).await {
        Ok(Some(folder)) => HttpResponse::Ok().json(folder),
        Ok(None) => HttpResponse::NotFound().body(format!(
            "Could not find record of folder with an id of {}",
            folder_id
        )),
        Err(e) => {
            tracing::error!("Failed to find folder: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}

#[get("/folders/{folderId}/subfolders")]
pub async fn get_all_subfolders(
    app_state: Data<AppState>,
    folder_id: Path<Uuid>,
) -> impl Responder {

    match app_state.folder_service.get_children_by_id(folder_id.into_inner()).await {
        Ok(folders) => {
            if folders.is_empty() {
                HttpResponse::NotFound().body("No subfolders were found")
            } else {
                HttpResponse::Ok().json(folders)
            }
        }
        Err(e) => {
            tracing::error!("Failed to fetch subfolders: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}

#[delete("/folders/{id}")]
pub async fn delete_folder(
    app_state: Data<AppState>,
    folder_id: Path<Uuid>,
) -> impl Responder {

    match app_state.folder_service.delete(folder_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("Failed to delete a folder: {:?}", e);
            map_data_err_to_http(e)
        }
    }
}


#[get("folders/{folderId}/file")]
pub async fn fetch_files_for_folder (
    app_state: Data<AppState>,
    id: Path<Uuid>
) -> impl Responder {

    let folder_id = id.into_inner();

    match app_state.folder_service.get_by_folder(folder_id).await {
        Ok(files) => {
            if files.is_empty() {
                HttpResponse::NotFound().body(format!("There were no files found for the given folder with id: {}", folder_id.clone()))
            } else {
                HttpResponse::Ok().json(files)
            }
        },
        Err(e) => {
            tracing::error!("Failed to fetch files inside a folder: {:?}", e);
            map_data_err_to_http(e)
        }

    }
}

#[patch("/folders/{id}/name")]
pub async fn rename_folder (
    app_state: Data<AppState>,
    folder_id: Path<Uuid>,
    req: Json<UpdateFolderNameCommand>
) -> impl Responder {
    let command: UpdateFolderNameCommand = req.into_inner();

    match app_state.folder_service.update_folder_name(command, folder_id.into_inner()).await {
        Ok(f) => {
            HttpResponse::Ok().json(f)
        },
        Err(e) => {
            tracing::error!("Failed to rename a folder: {}", e);
            map_data_err_to_http(e)
        }
    }
}

#[get("/folders/search")]
pub async fn search_folder (
    app_state: Data<AppState>,
    query: Query<SearchQuery>
) -> impl Responder {
    let search_term = query.into_inner().q;

    match app_state.folder_service.search_folder(search_term).await {
        Ok(f) => {
            if f.is_empty() {
                HttpResponse::Ok().body("No folders for the given search query")
            } else {
                HttpResponse::Ok().json(f)
            }
        },
        Err(e) => {
            tracing::error!("Failed to search for a folder: {}", e);
            map_data_err_to_http(e)
        }
    }
}

#[delete("/folders/all")]
pub async fn delete_chosen_folders (
    app_state: Data<AppState>,
    req: Json<DeleteChosenFoldersCommand>
) -> impl Responder {
    let command: DeleteChosenFoldersCommand = req.into_inner();

    match app_state.folder_service.delete_chosen_folders(&command.folder_ids).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            tracing::error!("Failed to delete chosen folders: {}", e);
            map_data_err_to_http(e)
        }
    }
}

#[get("/folders/{id}/files/filter")]
pub async fn filter_files_in_folder (
    app_state: Data<AppState>,
    req: Json<FilterFilesByFileTypeCommand>,
    folder_id: Path<Uuid>
) -> impl Responder {
    let command: FilterFilesByFileTypeCommand = req.into_inner();

    match app_state.folder_service.filter_files_by_folder(&command.file_types, folder_id.into_inner()).await {
        Ok(f) => {
            if f.is_empty() {
                HttpResponse::Ok().body("No files were found based on file types")
            } else {
                HttpResponse::Ok().json(f)
            }
        },

        Err(e) => {
            tracing::error!("Failed to filter out files in a folder: {}", e);
            map_data_err_to_http(e)
        }
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_root_folder);
    cfg.service(get_folder_by_id);
    cfg.service(get_all_subfolders);
    cfg.service(delete_folder);
    cfg.service(fetch_files_for_folder);
    cfg.service(rename_folder);
    cfg.service(search_folder);
    cfg.service(delete_chosen_folders);
    cfg.service(filter_files_in_folder);
}
