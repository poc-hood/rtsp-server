use gstreamer::glib;
use gstreamer_rtsp_server::prelude::*;

fn main() {
    gstreamer::init().unwrap();

    let server = gstreamer_rtsp_server::RTSPServer::default();

    let mounts = server.mount_points().unwrap();

    let factory = gstreamer_rtsp_server::RTSPMediaFactory::default();
    factory.set_launch(
        "videotestsrc ! videoconvert ! video/x-raw,width=640,height=480 ! x264enc ! rtph264pay name=pay0 pt=96"
    );
    println!("Pipeline set up: videotestsrc -> x264enc -> rtph264pay");

    factory.connect_media_configure(|_, media| {
        let pipeline = media.element();
        println!("Media configured: {:?}", pipeline);

        let bus = pipeline.bus().unwrap();
        bus.connect_message(Some("eos"), move |_, _| {
            println!("Received EOS (End of Stream) message. Restarting pipeline...");
            pipeline.set_state(gstreamer::State::Null).unwrap();
            pipeline.set_state(gstreamer::State::Playing).unwrap();
        });
    });

    mounts.add_factory("/test", factory);
    println!("RTSP mount point added at /test");

    let _ = server.attach(None);
    println!("Stream ready at rtsp://0.0.0.0:{}/test", server.service());
    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
}
