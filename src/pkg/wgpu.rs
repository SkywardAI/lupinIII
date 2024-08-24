use wgpu;

async fn run(){
    #[cfg_attr(target_arch="wasm32", allow(unused_variables))]
    let adapter={
        let instance=wgpu::Instance::default();
        #[cfg(not(target_arch="wasm32"))]{
            log::info!("Available adapters:");
            for a in instance.enumerate_adapters(wgpu::Backends::all()){
                log::info!("{:?}",a.get_info());
            }
        }
        if let Some(adapter)=instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await{
            log::info!("The adapter is avaliable");
            Some(adapter)
        }else{
            log::info!("The adapter is none");
            None
        }
    };
}


pub fn check_adapter(){
    #[cfg(not(target_arch="wasm32"))]
    {
        env_logger::builder()
            .filter(Some(module_path!()), log::LevelFilter::Info)
            .parse_default_env()
            .init();
        pollster::block_on(run());
    }
    #[cfg(target_arch="wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}