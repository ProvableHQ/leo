program test.aleo {

    async transition main() -> Future {
        assert_eq(self.signer, aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px);
        assert_eq(self.caller, aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px);

        // Setting the signer directly in a transition doesn't really make sense.
        // Note that while the signer is set, the caller remains unchanged because we are already in the context of a transition.
        // Setting the signer will come in handy once we support transaction generation and broadcast in a script.

        CheatCode::set_signer("APrivateKey1zkp3Y81VuFryWBXTJ4w4owHmMWq9tg35hN5tst1sYJv2N27");
        assert_eq(self.signer, aleo1wdune9hwf9ulz75rlus3sv5a978zfkezm8rnzk56qxrjg42gavrq8tz70m);
        assert_eq(self.caller, aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px);

        CheatCode::set_signer("APrivateKey1zkpJ5TGNDDiqSm71AcCpdarxZLHymvUFfkpAGprEUPQqDZp");
        assert_eq(self.signer, aleo1njwre8svpvn8jytkldyj3x45ewnks44qkuvasf290399768fayqs7h0yru);
        assert_eq(self.caller, aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px);

        return finish();
    }

    async function finish() {
        CheatCode::set_block_height(1u32);
        assert_eq(block.height, 1u32);
        CheatCode::set_block_height(999u32);
        assert_eq(block.height, 999u32);
    }
}
