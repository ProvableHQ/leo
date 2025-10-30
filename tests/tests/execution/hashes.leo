program test_hash_and_commit.aleo {
    struct S {
        x: field?,
        y: group?,
    }

    struct Outer {
        inner: S?,
        arr: [field?; 3]?,
    }

    transition hash(x: field, y: group) -> (public [field; 6], public group) {
        // Wrap x and y into optionals, include a none.
        let sx: field? = x;
        let sy: group? = none;

        let s: S? = S {
            x: sx,
            y: sy,
        };

        // Outer struct holding the optional s and an optional array
        let outer: Outer? = Outer {
            inner: s,
            arr: [1field, none, 3field],
        };

        // Use the outer optional in hashing
        return ([
            BHP256::hash_to_field(outer),
            BHP512::hash_to_field(outer),
            BHP768::hash_to_field(outer),
            BHP1024::hash_to_field(outer),
            Keccak256::hash_to_field(outer),
            Keccak384::hash_to_field(outer),
        ],
            Keccak256::hash_to_group([1u8, 2u8, 3u8, 4u8, 5u8, 6u8])
        );
    }

    transition commit(x: field, y: group, z: scalar) -> public [field; 6] {
        let sx: field? = none;
        let sy: group? = y;

        let s: S? = S {
            x: sx,
            y: sy,
        };

        let outer: Outer? = Outer {
            inner: s,
            arr: [10field, 20field, none],
        };

        return [
            BHP256::commit_to_field(outer, z),
            BHP512::commit_to_field(outer, z),
            BHP768::commit_to_field(outer, z),
            BHP1024::commit_to_field(outer, z),
            Pedersen64::commit_to_field(12u8, z),
            Pedersen128::commit_to_field(12u8, z),
        ];
    }

    @noupgrade
    async constructor() {}
}

// --- Test --- //

import test_hash_and_commit.aleo;

program test_test_hash_and_commit.aleo {

    @test
    transition hash_1() {
        let (fields, grp) = test_hash_and_commit.aleo/hash(123456field, 0group);
        // Fields extracted from execution trace
        assert_eq(fields, [
            2389033528541317015369294191168082905997398578750323243557863145864901166796field,
            172644632788497785943688808800949204936067994109484882301800054670053524265field,
            2935345460655404712415242713194718975328574533267387132400005962185865883341field,
            5817416973664279054460665977366739932224416429242942751719449180201781184059field,
            287759063810391575934384180273548695577458018507724744356046584908730543160field,
            3913727974389014614204978536390891835714340433349594459667742811322222489454field
        ]);
        assert_eq(grp, 1150511686575711078894586425189063917444782989726258642013897653550157939585group);
    }

    @test
    transition hash_2() {
        let (fields, grp) = test_hash_and_commit.aleo/hash(987654321field, 0group);
        assert_eq(fields, [
            6798576775415172930024795384318601958199326755638942990496387325229766236562field,
            4658232586824243767278641586542664532562957796225217469667907688947722388616field,
            7509757352560318781395278165510886076858703556666965559922627566156232816240field,
            6787929851299748200027290095196989629826553332067904143983285052766229554185field,
            8226379729174501730566792169755728656333931983456501340185779664235836140061field,
            2203427332685986822261672436628391935982043288069613336766290705077140103740field
        ]);
        assert_eq(grp, 1150511686575711078894586425189063917444782989726258642013897653550157939585group);
    }

    @test
    transition commit_1() {
        let fields = test_hash_and_commit.aleo/commit(123456field, 0group, 1scalar);
        assert_eq(fields, [
            795030275469581018113843893465458625686956867721830537891077681856251355681field,
            795030275469581018113843893465458625686956867721830537891077681856251355681field,
            795030275469581018113843893465458625686956867721830537891077681856251355681field,
            795030275469581018113843893465458625686956867721830537891077681856251355681field,
            177643203264384187118798439782220297144221145585119553422876357107962128695field,
            177643203264384187118798439782220297144221145585119553422876357107962128695field
        ]);
    }

    @test
    transition commit_2() {
        let fields = test_hash_and_commit.aleo/commit(987654321field, 0group, 1scalar);
        assert_eq(fields, [
            795030275469581018113843893465458625686956867721830537891077681856251355681field,
            795030275469581018113843893465458625686956867721830537891077681856251355681field,
            795030275469581018113843893465458625686956867721830537891077681856251355681field,
            795030275469581018113843893465458625686956867721830537891077681856251355681field,
            177643203264384187118798439782220297144221145585119553422876357107962128695field,
            177643203264384187118798439782220297144221145585119553422876357107962128695field
        ]);
    }

    @noupgrade
    async constructor() {}
}
