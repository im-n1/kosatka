use bollard::errors;
use bollard::image::{APIImages, ListImagesOptions, RemoveImageOptions, RemoveImageResults};
use bollard::Docker;

/// Lists all non-intermediate images.
pub async fn get_images(docker: Docker) -> Result<Vec<APIImages>, errors::Error> {
    Ok(docker
        .list_images(Some(ListImagesOptions::<String> {
            // all: true,
            ..Default::default()
        }))
        .await?)
}

/// Removes (force) the given image without a prune.
pub async fn delete_image(
    docker: Docker,
    id: &String,
) -> Result<Vec<RemoveImageResults>, errors::Error> {
    let options = RemoveImageOptions {
        force: true,
        noprune: true,
    };

    Ok(docker.remove_image(id, Some(options), None).await?)
    // Ok(vec![])
}
