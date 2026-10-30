# Specification intent — Stage 13

Status: ./verify.sh passed all groups (211/0, 237/0, 9/0). 42 intent statements, 112 supporting lemmas. This catalogue includes all named intent
lemmas and their supporting construction/induction lemmas; helpers are proof
obligations, not separate security guarantees. Counts overlap when included in
the transport root. No runtime changes.

Browser witnesses preserve every fixture header name and its order; each value
is reduced to one synthetic printable byte. Full fixture bytes remain covered
by the inherited loopback TLS tests. `reveal_protocol_literals!` exposes the
existing opaque Verus byte literals without adding assumptions. Per-header
helpers build each witness from the last header toward the first.

Rejection witnesses quantify over every valid policy, including policies that
drop every guarded name. The 4097-byte statement quantifies over every sequence
of that length. Universal claims use arbitrary input sequences and policies;
induction is on scan length or the header budget, never a finite input bound.
Header names in selected_spec are represented by the case-folded class IDs
from class_spec (0 Host, 2 Connection, 19–27 hard, 28–43 guarded, 44 unknown).

Config intent is **tested**, using the shipped files' exact bytes in
`tests/intent.rs::shipped_config_intent` and inherited policy tests.
Traversal intent is **tested**, in
`tests/intent.rs::traversal_paths_are_accepted_and_get_404`: GET/HEAD, strict/default,
1/7/1024-byte reads. Source review of `src/lib.rs::dispatch` establishes the
filesystem boundary: only exact `/` probes fixed binary paths; every other
unmatched path directly selects the fixed 404 reply. No request path reaches a
filesystem API. This is source review plus tests, not a filesystem theorem.

| Lemma | English statement | Location |
| --- | --- | --- |
| `reject_request4097` | All policies reject any 4097-byte input before interpreting its contents. | [intent.rs:276](crates/http-subset/src/intent.rs#L276) |
| `strict_checks_guarded` | A nonempty guarded-drop mask prevents strict mode, even when unknown names reject. | [intent.rs:283](crates/http-subset/src/intent.rs#L283) |
| `builtin_policy_intent` | The default rejects Cookie and accepts unknown names, with no guarded overrides. | [intent.rs:290](crates/http-subset/src/intent.rs#L290) |
| `accepted_request_line` | Every accepted request uses GET/HEAD and HTTP/1.0 or HTTP/1.1, and is at most 4096 bytes. | [intent.rs:298](crates/http-subset/src/intent.rs#L298) |
| `accepted_policy_intent` | Accepted header classes never include hard rejects and guarded names require their drop bit. | [intent.rs:308](crates/http-subset/src/intent.rs#L308) |
| `accepted_path_intent` | Every accepted path has only the path alphabet: percent/query/fragment/backslash/space are impossible. | [intent.rs:389](crates/http-subset/src/intent.rs#L389) |
| `accepted_values_and_count` | For every valid policy accepted requests have at most 24 headers, each with printable ASCII values. | [intent.rs:400](crates/http-subset/src/intent.rs#L400) |
| `accepted_exact_end` | Every accepted request ends at its first CRLFCRLF: no coalesced body or pipeline is accepted. | [intent.rs:544](crates/http-subset/src/intent.rs#L544) |
| `hard_name_intent` | Transfer-Encoding and Expect remain hard rejects under every valid policy, by their actual names. | [intent.rs:573](crates/http-subset/src/intent.rs#L573) |
| `witness_safari` | The built-in policy accepts the safari root header names in captured order. | [intent.rs:902](crates/http-subset/src/intent.rs#L902) |
| `witness_firefox` | The built-in policy accepts the firefox root header names in captured order. | [intent.rs:1277](crates/http-subset/src/intent.rs#L1277) |
| `witness_brave` | The built-in policy accepts the brave root header names in captured order. | [intent.rs:1810](crates/http-subset/src/intent.rs#L1810) |
| `witness_brave_cache_disabled` | The built-in policy accepts the brave cache disabled root header names in captured order. | [intent.rs:2371](crates/http-subset/src/intent.rs#L2371) |
| `witness_brave_dnt` | The built-in policy accepts the brave dnt root header names in captured order. | [intent.rs:2965](crates/http-subset/src/intent.rs#L2965) |
| `reject_headers25` | Every valid policy rejects the witness containing 25 Accept headers. | [intent.rs:3671](crates/http-subset/src/intent.rs#L3671) |
| `reject_cl_te` | Every valid policy rejects the root witness containing both Content-Length and Transfer-Encoding. | [intent.rs:3707](crates/http-subset/src/intent.rs#L3707) |
| `reject_te` | Every valid policy rejects the root witness containing Transfer-Encoding alone. | [intent.rs:3772](crates/http-subset/src/intent.rs#L3772) |
| `reject_cl_zero` | Every valid policy rejects the root witness carrying zero Content-Length. | [intent.rs:3827](crates/http-subset/src/intent.rs#L3827) |
| `reject_obs_space` | Every valid policy rejects the witness with a header line beginning with space. | [intent.rs:3881](crates/http-subset/src/intent.rs#L3881) |
| `reject_obs_tab` | Every valid policy rejects the witness with a header line beginning with tab. | [intent.rs:3928](crates/http-subset/src/intent.rs#L3928) |
| `reject_bare_lf` | Every valid policy rejects the witness using bare LF line endings. | [intent.rs:3975](crates/http-subset/src/intent.rs#L3975) |
| `reject_value_cr` | Every valid policy rejects the witness with a bare CR inside a value. | [intent.rs:4013](crates/http-subset/src/intent.rs#L4013) |
| `reject_value_nul` | Every valid policy rejects the witness with NUL inside a value. | [intent.rs:4063](crates/http-subset/src/intent.rs#L4063) |
| `reject_colon_space` | Every valid policy rejects the Host-name witness with whitespace before the colon. | [intent.rs:4113](crates/http-subset/src/intent.rs#L4113) |
| `reject_duplicate_host` | Every valid policy rejects the mixed-case duplicate Host witness. | [intent.rs:4161](crates/http-subset/src/intent.rs#L4161) |
| `reject_duplicate_connection` | Every valid policy rejects the mixed-case duplicate Connection witness. | [intent.rs:4227](crates/http-subset/src/intent.rs#L4227) |
| `reject_absolute` | Every valid policy rejects the absolute-form target witness. | [intent.rs:4293](crates/http-subset/src/intent.rs#L4293) |
| `reject_star` | Every valid policy rejects the asterisk target witness. | [intent.rs:4329](crates/http-subset/src/intent.rs#L4329) |
| `reject_query` | Every valid policy rejects the question-mark path witness. | [intent.rs:4365](crates/http-subset/src/intent.rs#L4365) |
| `reject_percent` | Every valid policy rejects the percent-sign path witness. | [intent.rs:4405](crates/http-subset/src/intent.rs#L4405) |
| `reject_fragment` | Every valid policy rejects the fragment-sign path witness. | [intent.rs:4445](crates/http-subset/src/intent.rs#L4445) |
| `reject_h2_preface` | Every valid policy rejects the HTTP/2 preface witness. | [intent.rs:4485](crates/http-subset/src/intent.rs#L4485) |
| `reject_lower_get` | Every valid policy rejects the lowercase-get witness. | [intent.rs:4521](crates/http-subset/src/intent.rs#L4521) |
| `reject_post` | Every valid policy rejects the POST witness. | [intent.rs:4556](crates/http-subset/src/intent.rs#L4556) |
| `reject_http12` | Every valid policy rejects the HTTP/1.2 witness. | [intent.rs:4591](crates/http-subset/src/intent.rs#L4591) |
| `reject_http20` | Every valid policy rejects the HTTP/2.0 witness. | [intent.rs:4630](crates/http-subset/src/intent.rs#L4630) |
| `reject_pipeline` | Every valid policy rejects the coalesced two-request pipeline witness. | [intent.rs:4669](crates/http-subset/src/intent.rs#L4669) |
| `reject_body` | Every valid policy rejects the witness with a trailing body byte. | [intent.rs:4718](crates/http-subset/src/intent.rs#L4718) |
| `reject_value257` | Every valid policy rejects the 257-byte-value witness. | [intent.rs:4763](crates/http-subset/src/intent.rs#L4763) |
| `reject_path257` | Every valid policy rejects the 257-byte-path witness. | [intent.rs:4877](crates/http-subset/src/intent.rs#L4877) |
| `reject_name65` | Every valid policy rejects the 65-byte-name witness. | [intent.rs:4981](crates/http-subset/src/intent.rs#L4981) |
| `reject_empty_value` | Every valid policy rejects the empty-value witness. | [intent.rs:5037](crates/http-subset/src/intent.rs#L5037) |
| `literal_bytes` | Expose the pinned verifier’s opaque byte literals without adding axioms. | [intent.rs:89](crates/http-subset/src/intent.rs#L89) |
| `empty_headers_intent` | One empty header terminator consumes the exact remaining input. | [intent.rs:171](crates/http-subset/src/intent.rs#L171) |
| `prepend_header_intent` | Construct one accepted header from a prefix, a nonempty bounded value and its proven tail. | [intent.rs:178](crates/http-subset/src/intent.rs#L178) |
| `forbidden_first_header` | One hard-reject class is enough to disallow a header list for every policy. | [intent.rs:201](crates/http-subset/src/intent.rs#L201) |
| `failed_header_tail` | A rejected recursive tail rejects its preceding header, without rescanning the rest. | [intent.rs:262](crates/http-subset/src/intent.rs#L262) |
| `path_scan_intent` | A path scan covers only path-alphabet bytes and stays within the input. | [intent.rs:319](crates/http-subset/src/intent.rs#L319) |
| `value_scan_intent` | A value scan covers printable ASCII bytes and stays within the input. | [intent.rs:329](crates/http-subset/src/intent.rs#L329) |
| `name_scan_intent` | A token scan contains no CR or LF and stays within the input. | [intent.rs:339](crates/http-subset/src/intent.rs#L339) |
| `header_values_intent` | Every accepted header list respects the count cap and has nonempty printable bounded values. | [intent.rs:354](crates/http-subset/src/intent.rs#L354) |
| `clean_folded_match` | Folding cannot hide CR or LF when the matched text excludes them. | [intent.rs:413](crates/http-subset/src/intent.rs#L413) |
| `known_prefix_clean` | A known header prefix contains no CR or LF, including its colon-space separator. | [intent.rs:426](crates/http-subset/src/intent.rs#L426) |
| `raw_prefix_clean` | Every accepted raw prefix, known or token-based, contains no CR or LF. | [intent.rs:493](crates/http-subset/src/intent.rs#L493) |
| `header_framing_intent` | Induction through every header places the sole double-CRLF at the exact end of input. | [intent.rs:512](crates/http-subset/src/intent.rs#L512) |
| `fixture_safari_header_10` | Construct header 11 and its tail for the safari witness. | [intent.rs:593](crates/http-subset/src/intent.rs#L593) |
| `fixture_safari_header_9` | Construct header 10 and its tail for the safari witness. | [intent.rs:622](crates/http-subset/src/intent.rs#L622) |
| `fixture_safari_header_8` | Construct header 9 and its tail for the safari witness. | [intent.rs:650](crates/http-subset/src/intent.rs#L650) |
| `fixture_safari_header_7` | Construct header 8 and its tail for the safari witness. | [intent.rs:678](crates/http-subset/src/intent.rs#L678) |
| `fixture_safari_header_6` | Construct header 7 and its tail for the safari witness. | [intent.rs:706](crates/http-subset/src/intent.rs#L706) |
| `fixture_safari_header_5` | Construct header 6 and its tail for the safari witness. | [intent.rs:734](crates/http-subset/src/intent.rs#L734) |
| `fixture_safari_header_4` | Construct header 5 and its tail for the safari witness. | [intent.rs:762](crates/http-subset/src/intent.rs#L762) |
| `fixture_safari_header_3` | Construct header 4 and its tail for the safari witness. | [intent.rs:790](crates/http-subset/src/intent.rs#L790) |
| `fixture_safari_header_2` | Construct header 3 and its tail for the safari witness. | [intent.rs:818](crates/http-subset/src/intent.rs#L818) |
| `fixture_safari_header_1` | Construct header 2 and its tail for the safari witness. | [intent.rs:846](crates/http-subset/src/intent.rs#L846) |
| `fixture_safari_header_0` | Construct header 1 and its tail for the safari witness. | [intent.rs:874](crates/http-subset/src/intent.rs#L874) |
| `fixture_firefox_header_11` | Construct header 12 and its tail for the firefox witness. | [intent.rs:940](crates/http-subset/src/intent.rs#L940) |
| `fixture_firefox_header_10` | Construct header 11 and its tail for the firefox witness. | [intent.rs:969](crates/http-subset/src/intent.rs#L969) |
| `fixture_firefox_header_9` | Construct header 10 and its tail for the firefox witness. | [intent.rs:997](crates/http-subset/src/intent.rs#L997) |
| `fixture_firefox_header_8` | Construct header 9 and its tail for the firefox witness. | [intent.rs:1025](crates/http-subset/src/intent.rs#L1025) |
| `fixture_firefox_header_7` | Construct header 8 and its tail for the firefox witness. | [intent.rs:1053](crates/http-subset/src/intent.rs#L1053) |
| `fixture_firefox_header_6` | Construct header 7 and its tail for the firefox witness. | [intent.rs:1081](crates/http-subset/src/intent.rs#L1081) |
| `fixture_firefox_header_5` | Construct header 6 and its tail for the firefox witness. | [intent.rs:1109](crates/http-subset/src/intent.rs#L1109) |
| `fixture_firefox_header_4` | Construct header 5 and its tail for the firefox witness. | [intent.rs:1137](crates/http-subset/src/intent.rs#L1137) |
| `fixture_firefox_header_3` | Construct header 4 and its tail for the firefox witness. | [intent.rs:1165](crates/http-subset/src/intent.rs#L1165) |
| `fixture_firefox_header_2` | Construct header 3 and its tail for the firefox witness. | [intent.rs:1193](crates/http-subset/src/intent.rs#L1193) |
| `fixture_firefox_header_1` | Construct header 2 and its tail for the firefox witness. | [intent.rs:1221](crates/http-subset/src/intent.rs#L1221) |
| `fixture_firefox_header_0` | Construct header 1 and its tail for the firefox witness. | [intent.rs:1249](crates/http-subset/src/intent.rs#L1249) |
| `fixture_brave_header_15` | Construct header 16 and its tail for the brave witness. | [intent.rs:1315](crates/http-subset/src/intent.rs#L1315) |
| `fixture_brave_header_14` | Construct header 15 and its tail for the brave witness. | [intent.rs:1344](crates/http-subset/src/intent.rs#L1344) |
| `fixture_brave_header_13` | Construct header 14 and its tail for the brave witness. | [intent.rs:1372](crates/http-subset/src/intent.rs#L1372) |
| `fixture_brave_header_12` | Construct header 13 and its tail for the brave witness. | [intent.rs:1400](crates/http-subset/src/intent.rs#L1400) |
| `fixture_brave_header_11` | Construct header 12 and its tail for the brave witness. | [intent.rs:1428](crates/http-subset/src/intent.rs#L1428) |
| `fixture_brave_header_10` | Construct header 11 and its tail for the brave witness. | [intent.rs:1456](crates/http-subset/src/intent.rs#L1456) |
| `fixture_brave_header_9` | Construct header 10 and its tail for the brave witness. | [intent.rs:1484](crates/http-subset/src/intent.rs#L1484) |
| `fixture_brave_header_8` | Construct header 9 and its tail for the brave witness. | [intent.rs:1512](crates/http-subset/src/intent.rs#L1512) |
| `fixture_brave_header_7` | Construct header 8 and its tail for the brave witness. | [intent.rs:1540](crates/http-subset/src/intent.rs#L1540) |
| `fixture_brave_header_6` | Construct header 7 and its tail for the brave witness. | [intent.rs:1568](crates/http-subset/src/intent.rs#L1568) |
| `fixture_brave_header_5` | Construct header 6 and its tail for the brave witness. | [intent.rs:1596](crates/http-subset/src/intent.rs#L1596) |
| `fixture_brave_header_4` | Construct header 5 and its tail for the brave witness. | [intent.rs:1643](crates/http-subset/src/intent.rs#L1643) |
| `fixture_brave_header_3` | Construct header 4 and its tail for the brave witness. | [intent.rs:1688](crates/http-subset/src/intent.rs#L1688) |
| `fixture_brave_header_2` | Construct header 3 and its tail for the brave witness. | [intent.rs:1726](crates/http-subset/src/intent.rs#L1726) |
| `fixture_brave_header_1` | Construct header 2 and its tail for the brave witness. | [intent.rs:1754](crates/http-subset/src/intent.rs#L1754) |
| `fixture_brave_header_0` | Construct header 1 and its tail for the brave witness. | [intent.rs:1782](crates/http-subset/src/intent.rs#L1782) |
| `fixture_brave_cache_disabled_header_16` | Construct header 17 and its tail for the brave cache disabled witness. | [intent.rs:1848](crates/http-subset/src/intent.rs#L1848) |
| `fixture_brave_cache_disabled_header_15` | Construct header 16 and its tail for the brave cache disabled witness. | [intent.rs:1877](crates/http-subset/src/intent.rs#L1877) |
| `fixture_brave_cache_disabled_header_14` | Construct header 15 and its tail for the brave cache disabled witness. | [intent.rs:1905](crates/http-subset/src/intent.rs#L1905) |
| `fixture_brave_cache_disabled_header_13` | Construct header 14 and its tail for the brave cache disabled witness. | [intent.rs:1933](crates/http-subset/src/intent.rs#L1933) |
| `fixture_brave_cache_disabled_header_12` | Construct header 13 and its tail for the brave cache disabled witness. | [intent.rs:1961](crates/http-subset/src/intent.rs#L1961) |
| `fixture_brave_cache_disabled_header_11` | Construct header 12 and its tail for the brave cache disabled witness. | [intent.rs:1989](crates/http-subset/src/intent.rs#L1989) |
| `fixture_brave_cache_disabled_header_10` | Construct header 11 and its tail for the brave cache disabled witness. | [intent.rs:2017](crates/http-subset/src/intent.rs#L2017) |
| `fixture_brave_cache_disabled_header_9` | Construct header 10 and its tail for the brave cache disabled witness. | [intent.rs:2045](crates/http-subset/src/intent.rs#L2045) |
| `fixture_brave_cache_disabled_header_8` | Construct header 9 and its tail for the brave cache disabled witness. | [intent.rs:2073](crates/http-subset/src/intent.rs#L2073) |
| `fixture_brave_cache_disabled_header_7` | Construct header 8 and its tail for the brave cache disabled witness. | [intent.rs:2101](crates/http-subset/src/intent.rs#L2101) |
| `fixture_brave_cache_disabled_header_6` | Construct header 7 and its tail for the brave cache disabled witness. | [intent.rs:2129](crates/http-subset/src/intent.rs#L2129) |
| `fixture_brave_cache_disabled_header_5` | Construct header 6 and its tail for the brave cache disabled witness. | [intent.rs:2176](crates/http-subset/src/intent.rs#L2176) |
| `fixture_brave_cache_disabled_header_4` | Construct header 5 and its tail for the brave cache disabled witness. | [intent.rs:2221](crates/http-subset/src/intent.rs#L2221) |
| `fixture_brave_cache_disabled_header_3` | Construct header 4 and its tail for the brave cache disabled witness. | [intent.rs:2259](crates/http-subset/src/intent.rs#L2259) |
| `fixture_brave_cache_disabled_header_2` | Construct header 3 and its tail for the brave cache disabled witness. | [intent.rs:2287](crates/http-subset/src/intent.rs#L2287) |
| `fixture_brave_cache_disabled_header_1` | Construct header 2 and its tail for the brave cache disabled witness. | [intent.rs:2315](crates/http-subset/src/intent.rs#L2315) |
| `fixture_brave_cache_disabled_header_0` | Construct header 1 and its tail for the brave cache disabled witness. | [intent.rs:2343](crates/http-subset/src/intent.rs#L2343) |
| `fixture_brave_dnt_header_17` | Construct header 18 and its tail for the brave dnt witness. | [intent.rs:2409](crates/http-subset/src/intent.rs#L2409) |
| `fixture_brave_dnt_header_16` | Construct header 17 and its tail for the brave dnt witness. | [intent.rs:2443](crates/http-subset/src/intent.rs#L2443) |
| `fixture_brave_dnt_header_15` | Construct header 16 and its tail for the brave dnt witness. | [intent.rs:2471](crates/http-subset/src/intent.rs#L2471) |
| `fixture_brave_dnt_header_14` | Construct header 15 and its tail for the brave dnt witness. | [intent.rs:2499](crates/http-subset/src/intent.rs#L2499) |
| `fixture_brave_dnt_header_13` | Construct header 14 and its tail for the brave dnt witness. | [intent.rs:2527](crates/http-subset/src/intent.rs#L2527) |
| `fixture_brave_dnt_header_12` | Construct header 13 and its tail for the brave dnt witness. | [intent.rs:2555](crates/http-subset/src/intent.rs#L2555) |
| `fixture_brave_dnt_header_11` | Construct header 12 and its tail for the brave dnt witness. | [intent.rs:2583](crates/http-subset/src/intent.rs#L2583) |
| `fixture_brave_dnt_header_10` | Construct header 11 and its tail for the brave dnt witness. | [intent.rs:2611](crates/http-subset/src/intent.rs#L2611) |
| `fixture_brave_dnt_header_9` | Construct header 10 and its tail for the brave dnt witness. | [intent.rs:2639](crates/http-subset/src/intent.rs#L2639) |
| `fixture_brave_dnt_header_8` | Construct header 9 and its tail for the brave dnt witness. | [intent.rs:2667](crates/http-subset/src/intent.rs#L2667) |
| `fixture_brave_dnt_header_7` | Construct header 8 and its tail for the brave dnt witness. | [intent.rs:2695](crates/http-subset/src/intent.rs#L2695) |
| `fixture_brave_dnt_header_6` | Construct header 7 and its tail for the brave dnt witness. | [intent.rs:2723](crates/http-subset/src/intent.rs#L2723) |
| `fixture_brave_dnt_header_5` | Construct header 6 and its tail for the brave dnt witness. | [intent.rs:2770](crates/http-subset/src/intent.rs#L2770) |
| `fixture_brave_dnt_header_4` | Construct header 5 and its tail for the brave dnt witness. | [intent.rs:2815](crates/http-subset/src/intent.rs#L2815) |
| `fixture_brave_dnt_header_3` | Construct header 4 and its tail for the brave dnt witness. | [intent.rs:2853](crates/http-subset/src/intent.rs#L2853) |
| `fixture_brave_dnt_header_2` | Construct header 3 and its tail for the brave dnt witness. | [intent.rs:2881](crates/http-subset/src/intent.rs#L2881) |
| `fixture_brave_dnt_header_1` | Construct header 2 and its tail for the brave dnt witness. | [intent.rs:2909](crates/http-subset/src/intent.rs#L2909) |
| `fixture_brave_dnt_header_0` | Construct header 1 and its tail for the brave dnt witness. | [intent.rs:2937](crates/http-subset/src/intent.rs#L2937) |
| `header25_suffix_24` | The suffix after 24 headers still contains more headers than the remaining budget. | [intent.rs:3000](crates/http-subset/src/intent.rs#L3000) |
| `header25_suffix_23` | The suffix after 23 headers still contains more headers than the remaining budget. | [intent.rs:3023](crates/http-subset/src/intent.rs#L3023) |
| `header25_suffix_22` | The suffix after 22 headers still contains more headers than the remaining budget. | [intent.rs:3050](crates/http-subset/src/intent.rs#L3050) |
| `header25_suffix_21` | The suffix after 21 headers still contains more headers than the remaining budget. | [intent.rs:3077](crates/http-subset/src/intent.rs#L3077) |
| `header25_suffix_20` | The suffix after 20 headers still contains more headers than the remaining budget. | [intent.rs:3104](crates/http-subset/src/intent.rs#L3104) |
| `header25_suffix_19` | The suffix after 19 headers still contains more headers than the remaining budget. | [intent.rs:3131](crates/http-subset/src/intent.rs#L3131) |
| `header25_suffix_18` | The suffix after 18 headers still contains more headers than the remaining budget. | [intent.rs:3158](crates/http-subset/src/intent.rs#L3158) |
| `header25_suffix_17` | The suffix after 17 headers still contains more headers than the remaining budget. | [intent.rs:3185](crates/http-subset/src/intent.rs#L3185) |
| `header25_suffix_16` | The suffix after 16 headers still contains more headers than the remaining budget. | [intent.rs:3212](crates/http-subset/src/intent.rs#L3212) |
| `header25_suffix_15` | The suffix after 15 headers still contains more headers than the remaining budget. | [intent.rs:3239](crates/http-subset/src/intent.rs#L3239) |
| `header25_suffix_14` | The suffix after 14 headers still contains more headers than the remaining budget. | [intent.rs:3266](crates/http-subset/src/intent.rs#L3266) |
| `header25_suffix_13` | The suffix after 13 headers still contains more headers than the remaining budget. | [intent.rs:3293](crates/http-subset/src/intent.rs#L3293) |
| `header25_suffix_12` | The suffix after 12 headers still contains more headers than the remaining budget. | [intent.rs:3320](crates/http-subset/src/intent.rs#L3320) |
| `header25_suffix_11` | The suffix after 11 headers still contains more headers than the remaining budget. | [intent.rs:3347](crates/http-subset/src/intent.rs#L3347) |
| `header25_suffix_10` | The suffix after 10 headers still contains more headers than the remaining budget. | [intent.rs:3374](crates/http-subset/src/intent.rs#L3374) |
| `header25_suffix_9` | The suffix after 9 headers still contains more headers than the remaining budget. | [intent.rs:3401](crates/http-subset/src/intent.rs#L3401) |
| `header25_suffix_8` | The suffix after 8 headers still contains more headers than the remaining budget. | [intent.rs:3428](crates/http-subset/src/intent.rs#L3428) |
| `header25_suffix_7` | The suffix after 7 headers still contains more headers than the remaining budget. | [intent.rs:3455](crates/http-subset/src/intent.rs#L3455) |
| `header25_suffix_6` | The suffix after 6 headers still contains more headers than the remaining budget. | [intent.rs:3482](crates/http-subset/src/intent.rs#L3482) |
| `header25_suffix_5` | The suffix after 5 headers still contains more headers than the remaining budget. | [intent.rs:3509](crates/http-subset/src/intent.rs#L3509) |
| `header25_suffix_4` | The suffix after 4 headers still contains more headers than the remaining budget. | [intent.rs:3536](crates/http-subset/src/intent.rs#L3536) |
| `header25_suffix_3` | The suffix after 3 headers still contains more headers than the remaining budget. | [intent.rs:3563](crates/http-subset/src/intent.rs#L3563) |
| `header25_suffix_2` | The suffix after 2 headers still contains more headers than the remaining budget. | [intent.rs:3590](crates/http-subset/src/intent.rs#L3590) |
| `header25_suffix_1` | The suffix after 1 headers still contains more headers than the remaining budget. | [intent.rs:3617](crates/http-subset/src/intent.rs#L3617) |
| `header25_suffix_0` | The suffix after 0 headers still contains more headers than the remaining budget. | [intent.rs:3644](crates/http-subset/src/intent.rs#L3644) |
