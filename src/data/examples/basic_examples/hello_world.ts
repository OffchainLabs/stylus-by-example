export const hello_world = `#[stylus_wax::main]
fn user_main(cmd: Cmd, msg: Msg) -> Response {
    console!("user_main() executing...");
    console!("cmd: {:?} --- msg: {:?}", cmd, msg);
    match cmd {
        TRANSFER_OWNERSHIP => run(transfer_ownership, msg),
        OWNER => run(owner, msg),
        _ => Ok(vec![]),
    }
}`;
