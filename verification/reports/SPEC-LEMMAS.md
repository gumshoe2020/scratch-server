# SPEC-LEMMAS.md: stage 13 (checking the spec against intent)

Branch `spec-lemmas`, built on `header-policy`. Approved by Andrew 2026-10-27 ("let's do it").
Authoritative for this stage.

## Why
The proofs show the code matches the spec. Nothing yet checks that the spec matches what we
*meant*. A wrong spec gets implemented faithfully and provably. This stage adds two checks on the
spec itself:

- **A. Intent lemmas:** short, readable facts about the spec, each proven by Verus.
- **B. Spec mutants:** deliberately wrong versions of the spec. Each one must be caught by an
  intent lemma or a test. A mutant that survives points to intent we never wrote down.

**This stage changes no runtime behaviour.** Executable code changes only if a lemma exposes a real
bug. In that case, stop, commit the evidence and explain it in NOTES.md. Don't fix it silently; a
fix needs Andrew's approval.

## A. Intent lemmas
Put them in a clearly separated section or module of the proven crate (for example `intent.rs`),
with one plain-English comment per lemma. Lemmas talk about the **spec functions**
(`request_spec`, `selected_spec`, `drops` and the config spec), not about executable functions.

1. **Witness acceptance (non-vacuity).** Under the built-in policy, the spec accepts the captured
   Safari, Firefox, Brave, Brave-cache-disabled and Brave+DNT root requests. Use the fixture bytes.
   If `by (compute)` or unfolding is impractically slow on full fixtures, use reduced fixtures that
   keep every header name and the order, and shorten the values. Record the reduction in NOTES.md.
2. **Witness rejection (attack corpus).** For **every valid policy** where the rejection doesn't
   depend on the policy, otherwise under the built-in policy, the spec rejects:
   - `Content-Length` plus `Transfer-Encoding` (CL.TE), `Transfer-Encoding: chunked` alone, and
     `Content-Length: 0`;
   - obs-fold (a header line starting with a space or tab);
   - a bare LF line ending, a bare CR inside a value, and NUL inside a value;
   - whitespace before the colon (`Host : x`);
   - duplicate `Host` and duplicate `Connection`, in mixed case;
   - absolute-form target (`GET http://x/ HTTP/1.1`), `*` target, `?` and `%` in the path, and `#`;
   - the HTTP/2 preface (`PRI * HTTP/2.0`), lowercase `get`, `POST`, `HTTP/1.2` and `HTTP/2.0`;
   - trailing bytes after the final CRLF (pipelining or a smuggled body);
   - a 257-byte value, a 257-byte path, a 65-byte header name, 25 headers and a 4097-byte request.
3. **"Never" statements over all inputs and all valid policies.** For every `s` and valid `p`, if
   `request_spec(s, p)` accepts, then:
   - the method is GET or HEAD, and the version is HTTP/1.0 or HTTP/1.1;
   - no header name folds to any hard-reject name;
   - no header name folds to a guarded name unless `p` drops that name;
   - there is at most one Host and at most one Connection;
   - every value byte is printable ASCII (no CR, LF, NUL or other control bytes);
   - every path byte is in the path set (no `%`, `?`, `#`, `\` or space);
   - the request ends exactly at the first CRLFCRLF (no trailing bytes);
   - the request is at most 4096 bytes, with at most 24 headers.
4. **Config intent.** The shipped `config/header-policy.conf` parses to exactly the built-in policy
   (prove it with the file's bytes, or test it if proving is impractical, and say which). The
   strict config parses to a policy with `strict()` true.
5. **Path traversal (open question).** The path grammar allows `.` and `/`, so `/../x` is accepted
   by the spec. The handler only serves exact routes (`/`, `/health`, ...). State this as a lemma
   or test: an accepted path containing `..` gets 404 and never reaches the filesystem (there is no
   filesystem access). If that can't be shown, record it as a finding and don't change behaviour.

Write `INTENT.md`: a table of every lemma, its one-line English statement and its location. This
is the file a human reads to judge whether the spec says what we mean.

## B. Spec mutants
Add `verification/scripts/spec-mutants.py`. It applies each mutant to a temporary copy of the repo (never the
working tree), runs `./verification/verify.sh` and the test suites, and records killed or survived.

The point is whether **intent** catches a wrong spec, so each mutant must change the spec **and**
the matching executable code consistently. The implementation-vs-spec proofs must still pass
(otherwise the mutant is malformed and must be fixed). A mutant counts as **killed** only when an
intent lemma, a fixture test or the differential test fails.

Mutants (at least these 15):
1. Remove one hard-reject name (`Transfer-Encoding`) from the hard class.
2. Move `Expect` from hard-reject to guarded.
3. Header cap 24 → 25.
4. Max value length 256 → 257.
5. Allow empty values.
6. Accept bare LF as a line ending.
7. Case-insensitive method matching (`get` accepted).
8. Accept `HTTP/1.2`.
9. Duplicate rule checks Host only (not Connection).
10. Built-in default `unknown reject` (should break the Brave+DNT witness).
11. Allow `%` in the path.
12. `strict()` ignores `guarded_drop`.
13. Token grammar accepts space.
14. Allow trailing bytes after the final CRLF.
15. Guarded `Cookie` defaults to drop.

Write `MUTANTS.md` with a table of mutant, killed?, and what killed it. For every survivor, either
add an intent lemma or test that kills it (then re-run and show it killed), or argue in writing why
it is equivalent or harmless and leave the decision to Andrew. Don't delete a mutant to make the
table look better.

## Acceptance
- `./verification/verify.sh` passes all proof groups, with the counts reported. There are no assume, admit,
  external_body, external or first-party unsafe.
- INTENT.md lists every lemma. Lemmas 1–3 exist and are proven, and 4–5 are proven or tested
  (say which).
- `verification/scripts/spec-mutants.py` runs end to end from a clean checkout. MUTANTS.md shows every mutant,
  and every survivor is either killed by a new check or argued in writing.
- All existing tests pass: the 100,000-case differential with 0 mismatches, the config fuzz and TLS
  acceptance.
- `git diff header-policy -- src crates/transport-core` is empty, and any change to executable
  code in `crates/http-subset` is listed and justified in NOTES.md (expected: none). formal-coverage
  is re-measured.
- Don't touch containers `scratch-server`, `nix-scratch` (8089), `parser-http` (8090),
  `parser-https` (8091) or `header-policy` (8095). No deployment is needed: the runtime binary
  should be unchanged.
