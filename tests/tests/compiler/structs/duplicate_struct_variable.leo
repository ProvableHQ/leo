
program test.aleo {
    // test 1: adjacent duplicate - no gap marker
    struct Bar {
        x: u32,
        x: u32,
    }

    // test 2: duplicate with gap marker - "..."
    struct Person {
        name: field,
        age: u8,
        city: field,
        name: field,
    }

    // test 3: multiple different duplicates
    struct Data {
        id: u32,
        val: u64,
        id: u32,
        val: u64,
    }

    // test 4: large gap between duplicates
    struct Config {
        host: field,
        port: u16,
        protocol: field,
        timeout: u32,
        retry_count: u8,
        max_connections: u32,
        buffer_size: u64,
        compression: bool,
        encryption: bool,
        host: field,
    }

    function main(zz: bool) -> bool {
        return true;
    }
}
