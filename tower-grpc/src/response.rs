use crate::metadata::MetadataMap;

/// A gRPC response and metadata from an RPC call.
#[derive(Debug)]
pub struct Response<T> {
    metadata: MetadataMap,
    message: T,
}

impl<T> Response<T> {
    /// Create a new gRPC response.
    pub fn new(message: T) -> Self {
        Response {
            metadata: MetadataMap::new(),
            message,
        }
    }

    /// Get a reference to the message
    pub fn get_ref(&self) -> &T {
        &self.message
    }

    /// Get a mutable reference to the message
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.message
    }

    /// Get a reference to the custom response metadata.
    pub fn metadata(&self) -> &MetadataMap {
        &self.metadata
    }

    /// Get a mutable reference to the response metadata.
    pub fn metadata_mut(&mut self) -> &mut MetadataMap {
        &mut self.metadata
    }

    /// Consumes `self`, returning the message
    pub fn into_inner(self) -> T {
        self.message
    }

    pub(crate) fn from_http(res: http::Response<T>) -> Self {
        let (head, message) = res.into_parts();
        Response {
            metadata: MetadataMap::from_headers(head.headers),
            message,
        }
    }

    pub fn into_http(self) -> http::Response<T> {
        let mut res = http::Response::new(self.message);

        *res.version_mut() = http::Version::HTTP_2;
        *res.headers_mut() = self.metadata.into_headers();

        res
    }

    pub fn map<F, U>(self, f: F) -> Response<U>
    where
        F: FnOnce(T) -> U,
    {
        let message = f(self.message);
        Response {
            metadata: self.metadata,
            message,
        }
    }

    // pub fn metadata()
    // pub fn metadata_bin()
}
