use errors;

/// This is the extractor module which should get the data from the web

#[async_trait::async_trait]
pub trait Extractor<T, R> {
    async fn fetch_data(&self, id: T) -> Result<R, errors::producer::ExtractorError>;
}
