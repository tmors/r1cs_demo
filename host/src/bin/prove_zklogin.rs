use risc0_zkvm::{default_prover, ExecutorEnv};
use zk_methods::ZKLOGIN_ELF;

fn main() {
    println!("this is prover zklogin");
    let jwt: &str = r#"{
        "iss": "https://accounts.google.com",
        "sub": "110169484474386276334",
        "aud": "your-client-id.apps.googleusercontent.com",
        "exp": 1783066337,
        "iat": 1783066337
    }"#;
    let iss: String = String::from("https://accounts.google.com");
    let sub: String = String::from("110169484474386276334");
    let aud: String = String::from("your-client-id.apps.googleusercontent.com");
    let exp: u64 = 1783066337;
    let iat: u64 = 1783066337;

    let env = ExecutorEnv::builder()
        .write(&jwt).unwrap()
        .write(&iss).unwrap()
        .write(&sub).unwrap()
        .write(&aud).unwrap()
        .write(&exp).unwrap()
        .write(&iat).unwrap()
        .build()
        .unwrap();

    // 获取 prover 并生成证明
    let prover = default_prover();
    let receipt = prover.prove(env, ZKLOGIN_ELF).unwrap();
    // 获取计算结果
    let _output: u32 = receipt.receipt.journal.decode().unwrap();
    let serialized = bincode::serialize(&receipt.receipt).unwrap();

    // Writing the serialized contect to receipt.bin file
    let _saved_file = match std::fs::write("./receipt_zklogin.bin", serialized) {
        Ok(()) => println!("Receipt saved and serialized as receipt.bin"),
        Err(_) => println!("Something went wrong !!"),

    };
    // 验证证明
    // receipt.receipt.verify(MULTIPLY_ID).unwrap();
}
