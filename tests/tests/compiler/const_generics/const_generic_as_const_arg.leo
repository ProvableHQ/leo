program const_generics.aleo {
    transition main() -> field {
        return abc::[0]() + abc::[1];
    }

    inline abc::[C: field]() -> field {
        return def::[C]();
    }

    inline def::[C: field]() -> field {
        return C;
    }
}
