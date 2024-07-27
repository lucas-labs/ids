use {
    bus::{Bus, BusReader},
    lool::cli::stylize::Stylize,
    rouille::{
        try_or_400,
        websocket::{start as start_ws, Websocket},
        Request, Response,
    },
    std::{
        sync::{Arc, Mutex},
        thread::spawn,
    },
};

pub fn handle(request: &Request, bus: Arc<Mutex<Bus<String>>>) -> Response {
    let (response, ws) = try_or_400!(start_ws::<String>(request, None));

    let peer = request.remote_addr().clone().to_string();

    spawn(move || {
        let ws = ws.recv().unwrap();
        let mut bus_reader = {
            let mut bus = bus.lock().unwrap();
            bus.add_rx()
        };
        websocket_handling_thread(ws, &mut bus_reader, peer);
    });

    response
}

fn websocket_handling_thread(mut ws: Websocket, bus_reader: &mut BusReader<String>, peer: String) {
    println!("» Accepted websocket connection from {}", peer.green());

    loop {
        match bus_reader.recv() {
            Ok(msg) => {
                if ws.send_text(&msg).is_err() {
                    break;
                }
            }
            Err(e) => println!("» Error receiving from reload_rcv: {:?}", e),
        }
    }

    println!("» Websocket connection with {} closed", peer.green());
}
