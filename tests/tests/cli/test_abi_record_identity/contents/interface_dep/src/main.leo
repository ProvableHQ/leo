import record_dep.aleo;

export interface ExternalRecords {
    fn echo_external(token: record_dep.aleo::Token) -> record_dep.aleo::Token;
}

program interface_dep.aleo {
    @noupgrade
    constructor() {}
}
