use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use thread_pool::ThreadPool;

fn main() {
    // 绑定本机的 7878 端口，返回一个 TcpListener 实例。如果绑定失败，程序会 panic!。
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    // 调用 incoming 方法来获取 TcpListener 实例的迭代器。这个迭代器会返回一系列的 TcpStream 实例，每一个都代表了一个与客户端的连接。
    for stream in listener.incoming() {
        // 如果成功获取了一个 TcpStream 实例，stream 变量就会被设置成 Ok 值，否则就会是 Err 值,程序会 panic!。
        let stream = stream.unwrap();

        // 此时浏览器访问会出现多个连接，因为浏览器会请求网站的各种资源，每一个资源都需要连接，所以会出现多个连接。
        pool.execute(|| handle_connection(stream)); // stream 实现了copy trait，所以可以多次使用
    }
}

// stream 定义为mut，因为它的内部状态会随着处理请求的过程而改变。
fn handle_connection(mut stream: TcpStream) {
    // // 定义一个缓冲区，用来存储从 TcpStream 实例中读取的数据。
    // let mut buffer = [0; 512];

    // // 读取 TcpStream 实例中的数据，并将结果存储到 buffer 中。如果读取成功，read 方法会返回 Ok 值，其中第二个元素代表读取的字节数。如果读取失败，read 方法会返回 Err 值。
    // stream.read(&mut buffer).unwrap();

    // // 将 buffer 中的字节转换成字符串，并打印出来。
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // 可以不指定缓冲区大小，直接读取
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|line| line.unwrap())
    //     .take_while(|line| !line.is_empty()) // 读取到空行就停止,浏览器通过连续发送两个换行符来代表一个 HTTP 请求的结束，所以为了从流中获取一个请求，我们获取行直到它们不为空。
    //     .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(content).unwrap();

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // let status_line = "HTTP/1.1 200 OK";

    // // // 将响应写入 TcpStream 实例，这样浏览器就能收到响应了。
    // // stream.write(response.as_bytes()).unwrap();

    // // 响应一个 html 文件
    // let contents = fs::read_to_string("hello.html").unwrap();

    // let content_length = contents.len();

    // let response = format!(
    //     "{status_line}\r\nContent-Length: {}\r\n\r\n{}",
    //     content_length, contents
    // );

    // stream.write(response.as_bytes()).unwrap();
}
