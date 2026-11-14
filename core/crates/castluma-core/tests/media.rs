use castluma_core::MediaSource;

#[test]
fn local_file_is_local() {
    let media_source = MediaSource::LocalFile {
        resource_id: String::from("selected-media-1"),
    };
    assert!(media_source.is_local())
}

#[test]
fn direct_url_is_not_local() {
    let media_source = MediaSource::DirectUrl {
        url: String::from("https://example.com/video.mp4"),
    };
    assert!(!media_source.is_local())
}
