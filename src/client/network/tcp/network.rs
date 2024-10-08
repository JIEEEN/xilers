use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Clone)]
pub struct TcpNetwork {
    // file transfer 요청을 받는 binding된 port
    pub listen_port: u16,
    file_storage: String,
}

// TODO: unwrap 처리
impl TcpNetwork {
    pub fn new(listen_port: u16, file_storage: String) -> Self {
        TcpNetwork {
            listen_port,
            file_storage,
        }
    }
    pub fn listener_init(&self) -> TcpListener {
        let addr = format!("{}:{}", "0.0.0.0".to_string(), self.listen_port);
        let listener = TcpListener::bind(addr).expect("이미 사용중인 포트입니다.");

        listener
    }

    pub fn connect(&self, peer_addr: String) -> Result<TcpStream, std::io::Error> {
        // file transfer 요청을 보내기 위한 연결
        TcpStream::connect(peer_addr)
    }

    pub fn send_request(&self, tcp_stream: &mut TcpStream, file_name: String) {
        tcp_stream.write_all(file_name.as_bytes()).unwrap();
        tcp_stream.flush().unwrap();

        let mut buf = [0u8; 1024];
        let sz = tcp_stream.read(&mut buf).unwrap();

        self.save_file(&buf[..sz], file_name);
    }

    fn save_file(&self, buffer: &[u8], file_name: String) {
        // TODO: 파일명과 관련해 추가 작업 필요 (저장)
        let payload_bytes = buffer;
        let path_file_name = std::path::Path::new(&file_name)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let abs_file_path = format!("{}/{}", &self.file_storage, path_file_name);
        let mut created_file = match std::fs::File::create(&abs_file_path) {
            Ok(file) => file,
            Err(_) => {
                println!("파일 경로를 확인해주시기 바랍니다: {}", abs_file_path);
                return;
            }
        };

        created_file.write(payload_bytes).unwrap();
        created_file.flush().unwrap();
    }

    pub fn listen(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = [0u8; 1024];
                    let peer_device_request = stream.read(&mut buf).unwrap();
                    let peer_device_request_str =
                        std::str::from_utf8(&buf[..peer_device_request]).unwrap();

                    let mut requested_file = std::fs::File::open(peer_device_request_str).unwrap();

                    let mut file_bytes = vec![0u8];
                    let data = requested_file.read_to_end(&mut file_bytes).unwrap();

                    stream.write(&file_bytes[..data]).unwrap();
                    stream.flush().unwrap();
                }
                Err(e) => {
                    eprintln!(
                        "Accept를 하는 과정에서 문제가 발생했습니다: {}",
                        e.to_string()
                    );
                }
            }
        }
    }
}
