; Leo Library
;
; Copyright (C) 2021 Aleo Systems Inc.
;
; Author: Eric McCarthy (bendyarm on GitHub)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(in-package "LEO")

;; The leo compiler tests are in
;;   leo/tests/compiler/<featuredir>/<testfile>.leo
;; One possible location for saved theorem templates could be
;;   leo-acl2/tests/compiler/<featuredir>/<testfile>.lisp
;; However, at first we will be creating them in a temporary location.

;; When certifying, the leo-acl2 directory might be changed.
;; These include-book forms expect that the :dir be set up to
;; find the leo-acl2 directory, for example:
;;    (add-include-book-dir :leo-acl2 "/home/yourname/leo-acl2/")

;; These books will be built in to the testing image, and the sources omitted,
;; so I commented them out for now.
;; TODO: make a variant of include-book that expands
;;       either to a no-op (for use in the testing image)
;;      or to a regular include-book (for use in a standard acl2 development environment)
;(include-book "test-json2ast" :dir :leo-acl2)
;(include-book "language/canonicalization" :dir :leo-acl2)
;(include-book "std/util/defconsts" :dir :system)


; Make this book dependent on the json files parsed below.
; The file names should be relative to the
; (depends-on "${initial-json-file}")
; (depends-on "${canonicalization-json-file}")

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Put ASTs in constants.

; Named constant for the initial AST.
(defconsts (& *initial-leo-ast* state)
  (jsonfile-to-formal "${initial-json-file}" state))

; Named constant for the canonicalized AST.
(defconsts (& *canonicalized-leo-ast* state)
  (jsonfile-to-formal "${canonicalization-json-file}" state))

; Additional constants for later compiler stages can be added here.

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

;; Theorem of correct canonicalization.

(defthm canonicalization-theorem
  (file-canop *initial-leo-ast*
              *canonicalized-leo-ast*))

; Additional theorems for later compiler stages can be added here.
