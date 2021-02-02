use crate::*;

#[derive(Debug, Default)]
pub(crate) struct Module {
    uuid: Uuid,

    name: String,
    binary_path: String,

    entry_point: Option<Index>,
    rebase_delta: i64,
    preferred_address: u64,


    sections: Vec<Index>,
    symbols: Vec<Index>,
    proxies: Vec<Index>,

}

impl Unique for Module {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<Module> {


}
