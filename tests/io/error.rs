#[test]
fn test_error() {
    assert_eq!(
        rustix::io::Errno::INVAL,
        rustix::io::Errno::from_raw_os_error(rustix::io::Errno::INVAL.raw_os_error())
    );
    assert_eq!(rustix::io::Errno::INVAL.raw_os_error(), libc::EINVAL);
}
