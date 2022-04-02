// 导入所需create
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    os::unix::prelude::AsRawFd,
    thread,
};

// 程序入口
fn main() {
    // 定义监听地址
    let addr = "127.0.0.1:7878";

    // 启动监听
    let listener = match TcpListener::bind(addr) {
        // 监听成功返回 TcpListener
        Ok(t) => t,
        // 失败 panic
        Err(error) => panic!("bind to address {} error {:?}", addr, error),
    };

    // 获取新的连接stream
    for stream in listener.incoming() {
        // 检查获取结果
        let stream = match stream {
            // 成功就继续处理
            Ok(s) => {
                println!("[{}]Connection established!", s.as_raw_fd());
                s
            }
            // 失败就返回
            Err(_) => {
                println!("incoming stream error");
                return;
            }
        };
        // 启动线程处理stream
        thread::spawn(|| {
            echo(stream);
        });
    }
}
// 处理一个TcpStream，返回收到的行
fn echo(stream: TcpStream) {
    // 获取一下id 方便在日志中区分
    let stream_id = stream.as_raw_fd();
    // 创建BufReader之前clone一下，作为writer
    let mut writer = stream
        .try_clone()
        .expect(format!("[{}]clone stream failed...", stream_id).as_str());

    // 创建BufReader 辅助读取读取
    let mut reader = BufReader::new(stream);

    // 循环
    loop {
        // 新建一个String 用来存放数据
        let mut line = String::new();

        // 读取一行数据
        match reader.read_line(&mut line) {
            // 错误（例如不是字符串）：读取下一行
            Err(err) => {
                // 打印错误
                println!("[{}]Read error: {}", stream_id, err);
                // 继续
                continue;
            }
            // 成功
            Ok(size) => {
                // 如果读取到的 size==0 说明连接关闭
                if size == 0 {
                    // 打印日志
                    println!("[{}]Connection Closed", stream_id);
                    // 退出循环
                    break;
                }
                // 打印日志
                println!("[{}]get line from client: {}", stream_id, line.trim_end());
            }
        }

        // 返回输入的行
        match writer.write(line.as_bytes()) {
            //成功打印一下日志
            Ok(_) => println!("[{}]return to client {}", stream_id, line.trim_end()),
            // 失败打印一下日志
            Err(e) => println!(
                "[{}]return to client {} error:{}",
                stream_id,
                line.trim_end(),
                e
            ),
        }
    }
}
