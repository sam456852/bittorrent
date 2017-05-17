use hyper::{Client, client, header};
use metainfo::MetaInfo;
use urlencoding::encode;

#[allow(dead_code)]
pub enum TrackerError {
    RetrievePeerError
}

/// Encodes parameters into a url
///
/// # Example
/// ```
/// # use tracker;
/// let params: Vec<(&str, &str)> = vec![("peer_id", "l33t"), ("port", "8080")];
/// assert_eq!("peer_id=l33t&port=8080".to_string(), parameterize(params));
/// ```
pub fn parameterize(parameters: Vec<(&str, &str)>) -> String {
    let query_params: Vec<String> = parameters.iter()
            .map(|&kv| format!("{}={}", kv.0, kv.1))
            .collect();

    query_params.join("&")
}

#[cfg(test)]
mod parameterize_tests {
    use super::parameterize;

    #[test]
    fn sample_params_test() {
        let params: Vec<(&str, &str)> = vec![("peer_id", "l33t"), ("port", "8080")];
        assert_eq!("peer_id=l33t&port=8080".to_string(), parameterize(params));
    }
}

/// Sends a request to the tracker specified by the MetaInfo's announce attribute and returns a
/// list of `peer`s and `peer_id`s.
pub fn retrieve_peers(metainfo: &MetaInfo, peer_id: &str, port: &str) -> Result<client::response::Response, TrackerError> {
    let uploaded = 0.to_string();
    let downloaded = 0.to_string();
    let left = metainfo.info.length.to_string();
    let compact = 1.to_string();

    let params: Vec<(&str, &str)> = vec![
        ("info_hash", metainfo.info_hash.as_str()),
        ("peer_id", peer_id),
        ("port", port),
        ("uploaded", uploaded.as_ref()),
        ("downloaded", downloaded.as_ref()),
        ("left", left.as_ref()),
        ("compact", compact.as_ref()),
        ("event", "started")
    ];
    let query_params = parameterize(params);
    let query_url = format!("{}?{}", metainfo.announce, query_params);
    println!("{}", query_url);
    let client = Client::new();

    match client.get(&query_url).header(header::Connection::close()).send() {
        Ok(response) => {
            // TODO: parse response body
            // return peers in response
            Ok(response)
        }
        Err(_) => Err(TrackerError::RetrievePeerError)
    }
}

#[test]
fn retrieve_peers_test() {
    use metainfo;
    let m = metainfo::from_file(&String::from("data/bsd.torrent")).unwrap();
    let res = retrieve_peers(&m, "tovatovatovatovatova", "8080");
    // println!("{:?}", res);
}
