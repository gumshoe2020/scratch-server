use super::*;

#[allow(unused_macros)]
macro_rules! reveal_protocol_literals {
    () => {
        reveal_byteslit(b" HTTP/1.0\r\n");
        reveal_byteslit(b" HTTP/1.1\r\n");
        reveal_byteslit(b": ");
        reveal_byteslit(b"Accept");
        reveal_byteslit(b"Accept-Encoding");
        reveal_byteslit(b"Accept-Encoding: ");
        reveal_byteslit(b"Accept-Language");
        reveal_byteslit(b"Accept-Language: ");
        reveal_byteslit(b"Accept: ");
        reveal_byteslit(b"Authorization");
        reveal_byteslit(b"Cache-Control");
        reveal_byteslit(b"Cache-Control: ");
        reveal_byteslit(b"Connection");
        reveal_byteslit(b"Connection: ");
        reveal_byteslit(b"Content-Length");
        reveal_byteslit(b"Cookie");
        reveal_byteslit(b"Expect");
        reveal_byteslit(b"Forwarded");
        reveal_byteslit(b"GET ");
        reveal_byteslit(b"HEAD ");
        reveal_byteslit(b"HTTP2-Settings");
        reveal_byteslit(b"Host");
        reveal_byteslit(b"Host: ");
        reveal_byteslit(b"If-Match");
        reveal_byteslit(b"If-Modified-Since");
        reveal_byteslit(b"If-None-Match");
        reveal_byteslit(b"If-Range");
        reveal_byteslit(b"If-Unmodified-Since");
        reveal_byteslit(b"Keep-Alive");
        reveal_byteslit(b"Max-Forwards");
        reveal_byteslit(b"Pragma");
        reveal_byteslit(b"Pragma: ");
        reveal_byteslit(b"Priority");
        reveal_byteslit(b"Priority: ");
        reveal_byteslit(b"Proxy-Authorization");
        reveal_byteslit(b"Proxy-Connection");
        reveal_byteslit(b"Range");
        reveal_byteslit(b"Referer");
        reveal_byteslit(b"Referer: ");
        reveal_byteslit(b"Sec-CH-UA");
        reveal_byteslit(b"Sec-CH-UA-Mobile");
        reveal_byteslit(b"Sec-CH-UA-Mobile: ");
        reveal_byteslit(b"Sec-CH-UA-Platform");
        reveal_byteslit(b"Sec-CH-UA-Platform: ");
        reveal_byteslit(b"Sec-CH-UA: ");
        reveal_byteslit(b"Sec-Fetch-Dest");
        reveal_byteslit(b"Sec-Fetch-Dest: ");
        reveal_byteslit(b"Sec-Fetch-Mode");
        reveal_byteslit(b"Sec-Fetch-Mode: ");
        reveal_byteslit(b"Sec-Fetch-Site");
        reveal_byteslit(b"Sec-Fetch-Site: ");
        reveal_byteslit(b"Sec-Fetch-User");
        reveal_byteslit(b"Sec-Fetch-User: ");
        reveal_byteslit(b"Sec-GPC");
        reveal_byteslit(b"Sec-GPC: ");
        reveal_byteslit(b"TE");
        reveal_byteslit(b"Trailer");
        reveal_byteslit(b"Transfer-Encoding");
        reveal_byteslit(b"Upgrade");
        reveal_byteslit(b"Upgrade-Insecure-Requests");
        reveal_byteslit(b"Upgrade-Insecure-Requests: ");
        reveal_byteslit(b"User-Agent");
        reveal_byteslit(b"User-Agent: ");
        reveal_byteslit(b"Via");
        reveal_byteslit(b"X-Forwarded-For");
        reveal_byteslit(b"X-Forwarded-Host");
        reveal_byteslit(b"X-Forwarded-Proto");
        reveal_byteslit(b"X-Real-IP");
        reveal_byteslit(b"\r\n");
        reveal_byteslit(b"drop");
        reveal_byteslit(b"guarded");
        reveal_byteslit(b"log-unknown-per-minute");
        reveal_byteslit(b"reject");
        reveal_byteslit(b"unknown");
    };
}

verus! {
pub open spec fn builtin_intent() -> Policy {
    Policy { unknown_drop: true, guarded_drop: 0, log_limit: 60 }
}

// Expose the pinned verifier’s opaque byte literals without adding axioms.
proof fn literal_bytes()
    ensures
        b" HTTP/1.0\r\n"@ == seq![32u8, 72u8, 84u8, 84u8, 80u8, 47u8, 49u8, 46u8, 48u8, 13u8, 10u8],
        b" HTTP/1.1\r\n"@ == seq![32u8, 72u8, 84u8, 84u8, 80u8, 47u8, 49u8, 46u8, 49u8, 13u8, 10u8],
        b": "@ == seq![58u8, 32u8],
        b"Accept"@ == seq![65u8, 99u8, 99u8, 101u8, 112u8, 116u8],
        b"Accept-Encoding"@ == seq![65u8, 99u8, 99u8, 101u8, 112u8, 116u8, 45u8, 69u8, 110u8, 99u8, 111u8, 100u8, 105u8, 110u8, 103u8],
        b"Accept-Encoding: "@ == seq![65u8, 99u8, 99u8, 101u8, 112u8, 116u8, 45u8, 69u8, 110u8, 99u8, 111u8, 100u8, 105u8, 110u8, 103u8, 58u8, 32u8],
        b"Accept-Language"@ == seq![65u8, 99u8, 99u8, 101u8, 112u8, 116u8, 45u8, 76u8, 97u8, 110u8, 103u8, 117u8, 97u8, 103u8, 101u8],
        b"Accept-Language: "@ == seq![65u8, 99u8, 99u8, 101u8, 112u8, 116u8, 45u8, 76u8, 97u8, 110u8, 103u8, 117u8, 97u8, 103u8, 101u8, 58u8, 32u8],
        b"Accept: "@ == seq![65u8, 99u8, 99u8, 101u8, 112u8, 116u8, 58u8, 32u8],
        b"Authorization"@ == seq![65u8, 117u8, 116u8, 104u8, 111u8, 114u8, 105u8, 122u8, 97u8, 116u8, 105u8, 111u8, 110u8],
        b"Cache-Control"@ == seq![67u8, 97u8, 99u8, 104u8, 101u8, 45u8, 67u8, 111u8, 110u8, 116u8, 114u8, 111u8, 108u8],
        b"Cache-Control: "@ == seq![67u8, 97u8, 99u8, 104u8, 101u8, 45u8, 67u8, 111u8, 110u8, 116u8, 114u8, 111u8, 108u8, 58u8, 32u8],
        b"Connection"@ == seq![67u8, 111u8, 110u8, 110u8, 101u8, 99u8, 116u8, 105u8, 111u8, 110u8],
        b"Connection: "@ == seq![67u8, 111u8, 110u8, 110u8, 101u8, 99u8, 116u8, 105u8, 111u8, 110u8, 58u8, 32u8],
        b"Content-Length"@ == seq![67u8, 111u8, 110u8, 116u8, 101u8, 110u8, 116u8, 45u8, 76u8, 101u8, 110u8, 103u8, 116u8, 104u8],
        b"Cookie"@ == seq![67u8, 111u8, 111u8, 107u8, 105u8, 101u8],
        b"Expect"@ == seq![69u8, 120u8, 112u8, 101u8, 99u8, 116u8],
        b"Forwarded"@ == seq![70u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 101u8, 100u8],
        b"GET "@ == seq![71u8, 69u8, 84u8, 32u8],
        b"HEAD "@ == seq![72u8, 69u8, 65u8, 68u8, 32u8],
        b"HTTP2-Settings"@ == seq![72u8, 84u8, 84u8, 80u8, 50u8, 45u8, 83u8, 101u8, 116u8, 116u8, 105u8, 110u8, 103u8, 115u8],
        b"Host"@ == seq![72u8, 111u8, 115u8, 116u8],
        b"Host: "@ == seq![72u8, 111u8, 115u8, 116u8, 58u8, 32u8],
        b"If-Match"@ == seq![73u8, 102u8, 45u8, 77u8, 97u8, 116u8, 99u8, 104u8],
        b"If-Modified-Since"@ == seq![73u8, 102u8, 45u8, 77u8, 111u8, 100u8, 105u8, 102u8, 105u8, 101u8, 100u8, 45u8, 83u8, 105u8, 110u8, 99u8, 101u8],
        b"If-None-Match"@ == seq![73u8, 102u8, 45u8, 78u8, 111u8, 110u8, 101u8, 45u8, 77u8, 97u8, 116u8, 99u8, 104u8],
        b"If-Range"@ == seq![73u8, 102u8, 45u8, 82u8, 97u8, 110u8, 103u8, 101u8],
        b"If-Unmodified-Since"@ == seq![73u8, 102u8, 45u8, 85u8, 110u8, 109u8, 111u8, 100u8, 105u8, 102u8, 105u8, 101u8, 100u8, 45u8, 83u8, 105u8, 110u8, 99u8, 101u8],
        b"Keep-Alive"@ == seq![75u8, 101u8, 101u8, 112u8, 45u8, 65u8, 108u8, 105u8, 118u8, 101u8],
        b"Max-Forwards"@ == seq![77u8, 97u8, 120u8, 45u8, 70u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 115u8],
        b"Pragma"@ == seq![80u8, 114u8, 97u8, 103u8, 109u8, 97u8],
        b"Pragma: "@ == seq![80u8, 114u8, 97u8, 103u8, 109u8, 97u8, 58u8, 32u8],
        b"Priority"@ == seq![80u8, 114u8, 105u8, 111u8, 114u8, 105u8, 116u8, 121u8],
        b"Priority: "@ == seq![80u8, 114u8, 105u8, 111u8, 114u8, 105u8, 116u8, 121u8, 58u8, 32u8],
        b"Proxy-Authorization"@ == seq![80u8, 114u8, 111u8, 120u8, 121u8, 45u8, 65u8, 117u8, 116u8, 104u8, 111u8, 114u8, 105u8, 122u8, 97u8, 116u8, 105u8, 111u8, 110u8],
        b"Proxy-Connection"@ == seq![80u8, 114u8, 111u8, 120u8, 121u8, 45u8, 67u8, 111u8, 110u8, 110u8, 101u8, 99u8, 116u8, 105u8, 111u8, 110u8],
        b"Range"@ == seq![82u8, 97u8, 110u8, 103u8, 101u8],
        b"Referer"@ == seq![82u8, 101u8, 102u8, 101u8, 114u8, 101u8, 114u8],
        b"Referer: "@ == seq![82u8, 101u8, 102u8, 101u8, 114u8, 101u8, 114u8, 58u8, 32u8],
        b"Sec-CH-UA"@ == seq![83u8, 101u8, 99u8, 45u8, 67u8, 72u8, 45u8, 85u8, 65u8],
        b"Sec-CH-UA-Mobile"@ == seq![83u8, 101u8, 99u8, 45u8, 67u8, 72u8, 45u8, 85u8, 65u8, 45u8, 77u8, 111u8, 98u8, 105u8, 108u8, 101u8],
        b"Sec-CH-UA-Mobile: "@ == seq![83u8, 101u8, 99u8, 45u8, 67u8, 72u8, 45u8, 85u8, 65u8, 45u8, 77u8, 111u8, 98u8, 105u8, 108u8, 101u8, 58u8, 32u8],
        b"Sec-CH-UA-Platform"@ == seq![83u8, 101u8, 99u8, 45u8, 67u8, 72u8, 45u8, 85u8, 65u8, 45u8, 80u8, 108u8, 97u8, 116u8, 102u8, 111u8, 114u8, 109u8],
        b"Sec-CH-UA-Platform: "@ == seq![83u8, 101u8, 99u8, 45u8, 67u8, 72u8, 45u8, 85u8, 65u8, 45u8, 80u8, 108u8, 97u8, 116u8, 102u8, 111u8, 114u8, 109u8, 58u8, 32u8],
        b"Sec-CH-UA: "@ == seq![83u8, 101u8, 99u8, 45u8, 67u8, 72u8, 45u8, 85u8, 65u8, 58u8, 32u8],
        b"Sec-Fetch-Dest"@ == seq![83u8, 101u8, 99u8, 45u8, 70u8, 101u8, 116u8, 99u8, 104u8, 45u8, 68u8, 101u8, 115u8, 116u8],
        b"Sec-Fetch-Dest: "@ == seq![83u8, 101u8, 99u8, 45u8, 70u8, 101u8, 116u8, 99u8, 104u8, 45u8, 68u8, 101u8, 115u8, 116u8, 58u8, 32u8],
        b"Sec-Fetch-Mode"@ == seq![83u8, 101u8, 99u8, 45u8, 70u8, 101u8, 116u8, 99u8, 104u8, 45u8, 77u8, 111u8, 100u8, 101u8],
        b"Sec-Fetch-Mode: "@ == seq![83u8, 101u8, 99u8, 45u8, 70u8, 101u8, 116u8, 99u8, 104u8, 45u8, 77u8, 111u8, 100u8, 101u8, 58u8, 32u8],
        b"Sec-Fetch-Site"@ == seq![83u8, 101u8, 99u8, 45u8, 70u8, 101u8, 116u8, 99u8, 104u8, 45u8, 83u8, 105u8, 116u8, 101u8],
        b"Sec-Fetch-Site: "@ == seq![83u8, 101u8, 99u8, 45u8, 70u8, 101u8, 116u8, 99u8, 104u8, 45u8, 83u8, 105u8, 116u8, 101u8, 58u8, 32u8],
        b"Sec-Fetch-User"@ == seq![83u8, 101u8, 99u8, 45u8, 70u8, 101u8, 116u8, 99u8, 104u8, 45u8, 85u8, 115u8, 101u8, 114u8],
        b"Sec-Fetch-User: "@ == seq![83u8, 101u8, 99u8, 45u8, 70u8, 101u8, 116u8, 99u8, 104u8, 45u8, 85u8, 115u8, 101u8, 114u8, 58u8, 32u8],
        b"Sec-GPC"@ == seq![83u8, 101u8, 99u8, 45u8, 71u8, 80u8, 67u8],
        b"Sec-GPC: "@ == seq![83u8, 101u8, 99u8, 45u8, 71u8, 80u8, 67u8, 58u8, 32u8],
        b"TE"@ == seq![84u8, 69u8],
        b"Trailer"@ == seq![84u8, 114u8, 97u8, 105u8, 108u8, 101u8, 114u8],
        b"Transfer-Encoding"@ == seq![84u8, 114u8, 97u8, 110u8, 115u8, 102u8, 101u8, 114u8, 45u8, 69u8, 110u8, 99u8, 111u8, 100u8, 105u8, 110u8, 103u8],
        b"Upgrade"@ == seq![85u8, 112u8, 103u8, 114u8, 97u8, 100u8, 101u8],
        b"Upgrade-Insecure-Requests"@ == seq![85u8, 112u8, 103u8, 114u8, 97u8, 100u8, 101u8, 45u8, 73u8, 110u8, 115u8, 101u8, 99u8, 117u8, 114u8, 101u8, 45u8, 82u8, 101u8, 113u8, 117u8, 101u8, 115u8, 116u8, 115u8],
        b"Upgrade-Insecure-Requests: "@ == seq![85u8, 112u8, 103u8, 114u8, 97u8, 100u8, 101u8, 45u8, 73u8, 110u8, 115u8, 101u8, 99u8, 117u8, 114u8, 101u8, 45u8, 82u8, 101u8, 113u8, 117u8, 101u8, 115u8, 116u8, 115u8, 58u8, 32u8],
        b"User-Agent"@ == seq![85u8, 115u8, 101u8, 114u8, 45u8, 65u8, 103u8, 101u8, 110u8, 116u8],
        b"User-Agent: "@ == seq![85u8, 115u8, 101u8, 114u8, 45u8, 65u8, 103u8, 101u8, 110u8, 116u8, 58u8, 32u8],
        b"Via"@ == seq![86u8, 105u8, 97u8],
        b"X-Forwarded-For"@ == seq![88u8, 45u8, 70u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 101u8, 100u8, 45u8, 70u8, 111u8, 114u8],
        b"X-Forwarded-Host"@ == seq![88u8, 45u8, 70u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 101u8, 100u8, 45u8, 72u8, 111u8, 115u8, 116u8],
        b"X-Forwarded-Proto"@ == seq![88u8, 45u8, 70u8, 111u8, 114u8, 119u8, 97u8, 114u8, 100u8, 101u8, 100u8, 45u8, 80u8, 114u8, 111u8, 116u8, 111u8],
        b"X-Real-IP"@ == seq![88u8, 45u8, 82u8, 101u8, 97u8, 108u8, 45u8, 73u8, 80u8],
        b"\r\n"@ == seq![13u8, 10u8],
        b"drop"@ == seq![100u8, 114u8, 111u8, 112u8],
        b"guarded"@ == seq![103u8, 117u8, 97u8, 114u8, 100u8, 101u8, 100u8],
        b"log-unknown-per-minute"@ == seq![108u8, 111u8, 103u8, 45u8, 117u8, 110u8, 107u8, 110u8, 111u8, 119u8, 110u8, 45u8, 112u8, 101u8, 114u8, 45u8, 109u8, 105u8, 110u8, 117u8, 116u8, 101u8],
        b"reject"@ == seq![114u8, 101u8, 106u8, 101u8, 99u8, 116u8],
        b"unknown"@ == seq![117u8, 110u8, 107u8, 110u8, 111u8, 119u8, 110u8],
{
    reveal_protocol_literals!();
}


// One empty header terminator consumes the exact remaining input.
proof fn empty_headers_intent(s: Seq<u8>, at: int, n: nat)
    requires matches(s, at, b"\r\n"@), at + 2 == s.len(),
    ensures raw_header_spec(s, at, n) == Some(Seq::<Header>::empty()),
        header_spec(s, at, n) == Some(Seq::<Header>::empty()),
{}

// Construct one accepted header from a prefix, a nonempty bounded value and its proven tail.
proof fn prepend_header_intent(s: Seq<u8>, at: int, n: nat, legacy: bool,
    id: u8, width: int, start: int, end: int, tail: Seq<Header>)
    requires n > 0, width > 0, 0 <= start < end <= s.len() <= usize::MAX,
        start == at + width, end - start <= MAX_VALUE,
        (if legacy { header_prefix(s, at) } else { raw_prefix_spec(s, at) }) == (id, width),
        value_end(s, start) == end, s[start] != b' ', s[end - 1] != b' ',
        matches(s, end, b"\r\n"@),
        intent_headers(s, end + 2, (n - 1) as nat, legacy) == Some(tail),
    ensures intent_headers(s, at, n, legacy) == Some(seq![Header { name: id,
        start: start as usize, end: end as usize }] + tail),
{}











// One hard-reject class is enough to disallow a header list for every policy.
proof fn forbidden_first_header(p: &Policy, h: Seq<Header>)
    requires h.len() > 0, 19 <= h[0].name < 28,
    ensures !allowed(p, h),
{
    assert(!drops(p, h[0].name));
    assert(!allowed(p, h));
}





















































// A rejected recursive tail rejects its preceding header, without rescanning the rest.
proof fn failed_header_tail(s: Seq<u8>, at: int, n: nat, legacy: bool)
    requires n > 0, 0 <= at <= s.len(),
        0 <= at + (if legacy { header_prefix(s, at) } else { raw_prefix_spec(s, at) }).1 <= s.len(),
        !(matches(s, at, b"\r\n"@) && at + 2 == s.len()),
        intent_headers(s, value_end(s, at + (if legacy { header_prefix(s, at) }
            else { raw_prefix_spec(s, at) }).1) + 2, (n - 1) as nat, legacy).is_none(),
    ensures intent_headers(s, at, n, legacy).is_none(),
{}





// All policies reject any 4097-byte input before interpreting its contents.
pub proof fn reject_request4097(s: Seq<u8>, p: &Policy)
    requires p.valid(), s.len() == 4097,
    ensures request_spec(s, p).is_none(),
{
    literal_bytes();}

// A nonempty guarded-drop mask prevents strict mode, even when unknown names reject.
pub proof fn strict_checks_guarded(p: &Policy)
    requires p.valid(), p.guarded_drop != 0,
    ensures !p.strict(),
{
    literal_bytes();}

// The default rejects Cookie and accepts unknown names, with no guarded overrides.
pub proof fn builtin_policy_intent()
    ensures !drops(&builtin_intent(), 28), drops(&builtin_intent(), 44),
        builtin_intent().guarded_drop == 0, !builtin_intent().strict(),
{
    assert((0u16 >> 0u8) & 1u16 == 0u16) by (bit_vector);
}

// Every accepted request uses GET/HEAD and HTTP/1.0 or HTTP/1.1, and is at most 4096 bytes.
pub proof fn accepted_request_line(s: Seq<u8>, p: &Policy)
    requires p.valid(), request_spec(s, p).is_some(),
    ensures s.len() <= 4096,
        matches(s, 0, b"GET "@) || matches(s, 0, b"HEAD "@),
        matches(s, request_spec(s, p).unwrap().2, b" HTTP/1.0\r\n"@)
            || matches(s, request_spec(s, p).unwrap().2, b" HTTP/1.1\r\n"@),
{
    literal_bytes();}

// Accepted header classes never include hard rejects and guarded names require their drop bit.
pub proof fn accepted_policy_intent(s: Seq<u8>, p: &Policy)
    requires p.valid(), request_spec(s, p).is_some(),
    ensures no_duplicates(selected_spec(s, p).unwrap().3),
        forall|i: int| 0 <= i < selected_spec(s, p).unwrap().3.len() ==>
            !(19 <= selected_spec(s, p).unwrap().3[i].name < 28)
            && (28 <= selected_spec(s, p).unwrap().3[i].name < 44 ==>
                drops(p, selected_spec(s, p).unwrap().3[i].name)),
{
    literal_bytes(); accepted_safety(s, p); }

// A path scan covers only path-alphabet bytes and stays within the input.
proof fn path_scan_intent(s: Seq<u8>, at: int)
    requires 0 <= at <= s.len(),
    ensures at <= path_end(s, at) <= s.len(),
        forall|j: int| at <= j < path_end(s, at) ==> path_char(s[j]),
    decreases s.len() - at,
{
    if at < s.len() && path_char(s[at]) { path_scan_intent(s, at + 1); }
}

// A value scan covers printable ASCII bytes and stays within the input.
proof fn value_scan_intent(s: Seq<u8>, at: int)
    requires 0 <= at <= s.len(),
    ensures at <= value_end(s, at) <= s.len(),
        forall|j: int| at <= j < value_end(s, at) ==> 32 <= #[trigger] s[j] <= 126,
    decreases s.len() - at,
{
    if at < s.len() && value_char(s[at]) { value_scan_intent(s, at + 1); }
}

// A token scan contains no CR or LF and stays within the input.
proof fn name_scan_intent(s: Seq<u8>, at: int)
    requires 0 <= at <= s.len(),
    ensures at <= name_end(s, at) <= s.len(),
        forall|j: int| at <= j < name_end(s, at) ==> #[trigger] s[j] != 13 && s[j] != 10,
    decreases s.len() - at,
{
    if at < s.len() && token(s[at]) { name_scan_intent(s, at + 1); }
}

pub open spec fn intent_headers(s: Seq<u8>, at: int, n: nat, legacy: bool) -> Option<Seq<Header>> {
    if legacy { header_spec(s, at, n) } else { raw_header_spec(s, at, n) }
}

// Every accepted header list respects the count cap and has nonempty printable bounded values.
#[verifier::rlimit(50)]
proof fn header_values_intent(s: Seq<u8>, at: int, n: nat, legacy: bool)
    requires 0 <= at <= s.len() <= 4096, n <= 24, intent_headers(s, at, n, legacy).is_some(),
    ensures intent_headers(s, at, n, legacy).unwrap().len() <= n,
        forall|i: int| 0 <= i < intent_headers(s, at, n, legacy).unwrap().len() ==>
            0 <= intent_headers(s, at, n, legacy).unwrap()[i].start
                < intent_headers(s, at, n, legacy).unwrap()[i].end <= s.len()
            && intent_headers(s, at, n, legacy).unwrap()[i].end
                - intent_headers(s, at, n, legacy).unwrap()[i].start <= 256
            && (forall|j: int| intent_headers(s, at, n, legacy).unwrap()[i].start <= j
                < intent_headers(s, at, n, legacy).unwrap()[i].end ==> 32 <= #[trigger] s[j] <= 126),
    decreases n,
{
    reveal_byteslit(b"\r\n");
    if !(matches(s, at, b"\r\n"@) && at + 2 == s.len()) {
        let prefix = if legacy { header_prefix(s, at) } else { raw_prefix_spec(s, at) };
        if !legacy && header_prefix(s, at).1 == 0 { name_scan_intent(s, at); }
        let start = at + prefix.1;
        assert(0 <= start <= s.len());
        value_scan_intent(s, start);
        let end = value_end(s, start);
        header_values_intent(s, end + 2, (n - 1) as nat, legacy);
        let h = intent_headers(s, at, n, legacy).unwrap();
        let tail = intent_headers(s, end + 2, (n - 1) as nat, legacy).unwrap();
        assert forall|i: int| 0 <= i < h.len() implies
            0 <= h[i].start < h[i].end <= s.len()
            && h[i].end - h[i].start <= 256
            && (forall|j: int| h[i].start <= j < h[i].end ==> 32 <= #[trigger] s[j] <= 126) by {
            if i == 0 {
                assert(h[i].start == start && h[i].end == end);
            } else { assert(h[i] == tail[i - 1]); }
        }
    }
}

// Every accepted path has only the path alphabet: percent/query/fragment/backslash/space are impossible.
pub proof fn accepted_path_intent(s: Seq<u8>, p: &Policy)
    requires p.valid(), request_spec(s, p).is_some(),
    ensures forall|j: int| request_spec(s, p).unwrap().1 <= j < request_spec(s, p).unwrap().2 ==>
        path_char(s[j]) && s[j] != b'%' && s[j] != b'?' && s[j] != b'#'
            && s[j] != b'\\' && s[j] != b' ',
{
    let start = if matches(s, 0, b"HEAD "@) { 5int } else { 4int };
    path_scan_intent(s, start);
}

// For every valid policy accepted requests have at most 24 headers, each with printable ASCII values.
pub proof fn accepted_values_and_count(s: Seq<u8>, p: &Policy)
    requires p.valid(), request_spec(s, p).is_some(),
    ensures selected_spec(s, p).unwrap().3.len() <= 24,
        forall|i: int, j: int| 0 <= i < selected_spec(s, p).unwrap().3.len()
            && #[trigger] selected_spec(s, p).unwrap().3[i].start <= j < selected_spec(s, p).unwrap().3[i].end
            ==> 32 <= #[trigger] s[j] <= 126,
{
    let start = if matches(s, 0, b"HEAD "@) { 5int } else { 4int };
    path_scan_intent(s, start);
    header_values_intent(s, path_end(s, start) + 11, MAX_HEADERS as nat, p.strict());
}

// Folding cannot hide CR or LF when the matched text excludes them.
proof fn clean_folded_match(s: Seq<u8>, at: int, text: Seq<u8>)
    requires matches_header(s, at, text),
        forall|i: int| 0 <= i < text.len() ==> ascii_lower(text[i]) != 13 && ascii_lower(text[i]) != 10,
    ensures 0 <= at <= at + text.len() <= s.len(),
        forall|j: int| at <= j < at + text.len() ==> #[trigger] s[j] != 13 && s[j] != 10,
{
    assert forall|j: int| at <= j < at + text.len() implies s[j] != 13 && s[j] != 10 by {
        assert(ascii_lower(s[j]) == ascii_lower(text[j - at]));
    }
}

// A known header prefix contains no CR or LF, including its colon-space separator.
#[verifier::rlimit(30)]
proof fn known_prefix_clean(s: Seq<u8>, at: int)
    requires header_prefix(s, at).1 > 0,
    ensures 0 <= at < at + header_prefix(s, at).1 <= s.len(),
        forall|j: int| at <= j < at + header_prefix(s, at).1 ==>
            #[trigger] s[j] != 13 && s[j] != 10,
{
    literal_bytes();
    if matches_header(s, at, b"Host: "@) {
        clean_folded_match(s, at, b"Host: "@);
    }
    else if matches_header(s, at, b"Accept: "@) {
        clean_folded_match(s, at, b"Accept: "@);
    }
    else if matches_header(s, at, b"Connection: "@) {
        clean_folded_match(s, at, b"Connection: "@);
    }
    else if matches_header(s, at, b"User-Agent: "@) {
        clean_folded_match(s, at, b"User-Agent: "@);
    }
    else if matches_header(s, at, b"Accept-Language: "@) {
        clean_folded_match(s, at, b"Accept-Language: "@);
    }
    else if matches_header(s, at, b"Accept-Encoding: "@) {
        clean_folded_match(s, at, b"Accept-Encoding: "@);
    }
    else if matches_header(s, at, b"Upgrade-Insecure-Requests: "@) {
        clean_folded_match(s, at, b"Upgrade-Insecure-Requests: "@);
    }
    else if matches_header(s, at, b"Priority: "@) {
        clean_folded_match(s, at, b"Priority: "@);
    }
    else if matches_header(s, at, b"Referer: "@) {
        clean_folded_match(s, at, b"Referer: "@);
    }
    else if matches_header(s, at, b"Sec-Fetch-Dest: "@) {
        clean_folded_match(s, at, b"Sec-Fetch-Dest: "@);
    }
    else if matches_header(s, at, b"Sec-Fetch-Site: "@) {
        clean_folded_match(s, at, b"Sec-Fetch-Site: "@);
    }
    else if matches_header(s, at, b"Sec-Fetch-Mode: "@) {
        clean_folded_match(s, at, b"Sec-Fetch-Mode: "@);
    }
    else if matches_header(s, at, b"Sec-Fetch-User: "@) {
        clean_folded_match(s, at, b"Sec-Fetch-User: "@);
    }
    else if matches_header(s, at, b"Cache-Control: "@) {
        clean_folded_match(s, at, b"Cache-Control: "@);
    }
    else if matches_header(s, at, b"Sec-GPC: "@) {
        clean_folded_match(s, at, b"Sec-GPC: "@);
    }
    else if matches_header(s, at, b"Sec-CH-UA: "@) {
        clean_folded_match(s, at, b"Sec-CH-UA: "@);
    }
    else if matches_header(s, at, b"Sec-CH-UA-Mobile: "@) {
        clean_folded_match(s, at, b"Sec-CH-UA-Mobile: "@);
    }
    else if matches_header(s, at, b"Sec-CH-UA-Platform: "@) {
        clean_folded_match(s, at, b"Sec-CH-UA-Platform: "@);
    }
    else if matches_header(s, at, b"Pragma: "@) {
        clean_folded_match(s, at, b"Pragma: "@);
    }
}

// Every accepted raw prefix, known or token-based, contains no CR or LF.
proof fn raw_prefix_clean(s: Seq<u8>, at: int)
    requires 0 <= at <= s.len(), raw_prefix_spec(s, at).1 > 0,
    ensures at < at + raw_prefix_spec(s, at).1 <= s.len(),
        forall|j: int| at <= j < at + raw_prefix_spec(s, at).1 ==>
            #[trigger] s[j] != 13 && s[j] != 10,
{
    reveal_byteslit(b": ");
    if header_prefix(s, at).1 > 0 { known_prefix_clean(s, at); }
    else {
        name_scan_intent(s, at);
        let end = name_end(s, at);
        assert forall|j: int| at <= j < end + 2 implies s[j] != 13 && s[j] != 10 by {
            if j >= end { assert(s[j] == b": "@[j - end]); }
        }
    }
}

// Induction through every header places the sole double-CRLF at the exact end of input.
#[verifier::rlimit(30)]
proof fn header_framing_intent(s: Seq<u8>, at: int, n: nat, legacy: bool)
    requires 2 <= at <= s.len() <= 4096, n <= 24,
        matches(s, at - 2, b"\r\n"@), intent_headers(s, at, n, legacy).is_some(),
    ensures matches(s, s.len() - 4, b"\r\n\r\n"@),
        forall|j: int| at - 2 <= j && matches(s, j, b"\r\n\r\n"@) ==> j == s.len() - 4,
    decreases n,
{
    reveal_byteslit(b"\r\n"); reveal_byteslit(b"\r\n\r\n");
    if matches(s, at, b"\r\n"@) && at + 2 == s.len() {
        assert(s.subrange(s.len() - 4, s.len() as int) =~= b"\r\n\r\n"@);
    } else {
        let prefix = if legacy { header_prefix(s, at) } else { raw_prefix_spec(s, at) };
        if legacy { known_prefix_clean(s, at); } else { raw_prefix_clean(s, at); }
        let start = at + prefix.1;
        value_scan_intent(s, start);
        let end = value_end(s, start);
        header_framing_intent(s, end + 2, (n - 1) as nat, legacy);
        assert forall|j: int| at - 2 <= j && matches(s, j, b"\r\n\r\n"@)
            implies j == s.len() - 4 by {
            if j < end {
                if j < at {
                    assert(at <= j + 2 < start);
                    assert(s[j + 2] == 13);
                } else if j < start { assert(s[j] == 13); }
                else { assert(s[j] == 13); }
            }
        }
    }
}

// Every accepted request ends at its first CRLFCRLF: no coalesced body or pipeline is accepted.
#[verifier::rlimit(30)]
pub proof fn accepted_exact_end(s: Seq<u8>, p: &Policy)
    requires p.valid(), request_spec(s, p).is_some(),
    ensures matches(s, s.len() - 4, b"\r\n\r\n"@),
        forall|j: int| 0 <= j && matches(s, j, b"\r\n\r\n"@) ==> j == s.len() - 4,
{
    literal_bytes(); reveal_byteslit(b"\r\n\r\n");
    let start = if matches(s, 0, b"HEAD "@) { 5int } else { 4int };
    let end = path_end(s, start);
    path_scan_intent(s, start);
    assert(matches(s, end + 9, b"\r\n"@)) by {
        assert(s.subrange(end + 9, end + 11) =~= b"\r\n"@);
    }
    header_framing_intent(s, end + 11, MAX_HEADERS as nat, p.strict());
    assert forall|j: int| 0 <= j && matches(s, j, b"\r\n\r\n"@)
        implies j == s.len() - 4 by {
        if j < end + 9 {
            if j < start {
                if matches(s, 0, b"HEAD "@) { assert(s[j] == b"HEAD "@[j]); }
                else { assert(s[j] == b"GET "@[j]); }
            } else if j < end { assert(path_char(s[j])); }
            else {
                if matches(s, end, b" HTTP/1.0\r\n"@) { assert(s[j] == b" HTTP/1.0\r\n"@[j - end]); }
                else { assert(s[j] == b" HTTP/1.1\r\n"@[j - end]); }
            }
            assert(s[j] != 13);
        }
    }
}
// Transfer-Encoding and Expect remain hard rejects under every valid policy, by their actual names.
pub proof fn hard_name_intent(p: &Policy)
    requires p.valid(),
    ensures !drops(p, class_spec(b"Transfer-Encoding"@)),
        !drops(p, class_spec(b"Expect"@)),
{
    literal_bytes();
    assert(class_spec(b"Transfer-Encoding"@) == 20);
    assert(class_spec(b"Expect"@) == 23);
}
}

// Independent proof module: safari.
mod safari {
    use super::*;
    verus! {
// Reduced safari fixture: original names/order and single-byte synthetic values.
pub open spec fn fixture_safari() -> Seq<u8> { b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n"@ }

// Construct header 11 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_10()
    ensures raw_header_spec(fixture_safari(), 173, 14) == Some(seq![Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    assert(s.subrange(188, 190) =~= b"\r\n"@);
    empty_headers_intent(s, 188, 13);
    assert(raw_prefix_spec(s, 173) == (2u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 185) == 186) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(186, 188) =~= b"\r\n"@);
    prepend_header_intent(s, 173, 14, false, 2, 12, 185, 186, seq![]);
    assert(raw_header_spec(s, 173, 14).unwrap() =~= seq![Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 10 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_9()
    ensures raw_header_spec(fixture_safari(), 153, 15) == Some(seq![Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_10();
    assert(raw_prefix_spec(s, 153) == (5u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 170) == 171) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(171, 173) =~= b"\r\n"@);
    prepend_header_intent(s, 153, 15, false, 5, 17, 170, 171, seq![Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 153, 15).unwrap() =~= seq![Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 9 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_8()
    ensures raw_header_spec(fixture_safari(), 140, 16) == Some(seq![Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_9();
    assert(raw_prefix_spec(s, 140) == (7u8, 10int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 150) == 151) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(151, 153) =~= b"\r\n"@);
    prepend_header_intent(s, 140, 16, false, 7, 10, 150, 151, seq![Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 140, 16).unwrap() =~= seq![Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 8 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_7()
    ensures raw_header_spec(fixture_safari(), 120, 17) == Some(seq![Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_8();
    assert(raw_prefix_spec(s, 120) == (4u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 137) == 138) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(138, 140) =~= b"\r\n"@);
    prepend_header_intent(s, 120, 17, false, 4, 17, 137, 138, seq![Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 120, 17).unwrap() =~= seq![Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 7 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_6()
    ensures raw_header_spec(fixture_safari(), 101, 18) == Some(seq![Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_7();
    assert(raw_prefix_spec(s, 101) == (11u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 117) == 118) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(118, 120) =~= b"\r\n"@);
    prepend_header_intent(s, 101, 18, false, 11, 16, 117, 118, seq![Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 101, 18).unwrap() =~= seq![Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 6 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_5()
    ensures raw_header_spec(fixture_safari(), 82, 19) == Some(seq![Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_6();
    assert(raw_prefix_spec(s, 82) == (10u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 98) == 99) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(99, 101) =~= b"\r\n"@);
    prepend_header_intent(s, 82, 19, false, 10, 16, 98, 99, seq![Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 82, 19).unwrap() =~= seq![Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 5 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_4()
    ensures raw_header_spec(fixture_safari(), 70, 20) == Some(seq![Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_5();
    assert(raw_prefix_spec(s, 70) == (8u8, 9int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 79) == 80) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(80, 82) =~= b"\r\n"@);
    prepend_header_intent(s, 70, 20, false, 8, 9, 79, 80, seq![Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 70, 20).unwrap() =~= seq![Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 4 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_3()
    ensures raw_header_spec(fixture_safari(), 59, 21) == Some(seq![Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_4();
    assert(raw_prefix_spec(s, 59) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 67) == 68) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(68, 70) =~= b"\r\n"@);
    prepend_header_intent(s, 59, 21, false, 1, 8, 67, 68, seq![Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 59, 21).unwrap() =~= seq![Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 3 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_2()
    ensures raw_header_spec(fixture_safari(), 44, 22) == Some(seq![Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_3();
    assert(raw_prefix_spec(s, 44) == (3u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 56) == 57) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(57, 59) =~= b"\r\n"@);
    prepend_header_intent(s, 44, 22, false, 3, 12, 56, 57, seq![Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 44, 22).unwrap() =~= seq![Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 2 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_1()
    ensures raw_header_spec(fixture_safari(), 25, 23) == Some(seq![Header { name: 9, start: 41, end: 42 }, Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_2();
    assert(raw_prefix_spec(s, 25) == (9u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 41) == 42) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(42, 44) =~= b"\r\n"@);
    prepend_header_intent(s, 25, 23, false, 9, 16, 41, 42, seq![Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 25, 23).unwrap() =~= seq![Header { name: 9, start: 41, end: 42 }, Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// Construct header 1 and its tail for the safari witness.
#[verifier::rlimit(20)]
proof fn fixture_safari_header_0()
    ensures raw_header_spec(fixture_safari(), 16, 24) == Some(seq![Header { name: 0, start: 22, end: 23 }, Header { name: 9, start: 41, end: 42 }, Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_1();
    assert(raw_prefix_spec(s, 16) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(23, 25) =~= b"\r\n"@);
    prepend_header_intent(s, 16, 24, false, 0, 6, 22, 23, seq![Header { name: 9, start: 41, end: 42 }, Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
    assert(raw_header_spec(s, 16, 24).unwrap() =~= seq![Header { name: 0, start: 22, end: 23 }, Header { name: 9, start: 41, end: 42 }, Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]);
}

// The built-in policy accepts the safari root header names in captured order.
#[verifier::rlimit(20)]
pub proof fn witness_safari()
    ensures request_spec(fixture_safari(), &builtin_intent()).is_some(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_safari();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nSec-Fetch-Dest: x\r\nUser-Agent: x\r\nAccept: x\r\nReferer: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nAccept-Language: x\r\nPriority: x\r\nAccept-Encoding: x\r\nConnection: x\r\n\r\n");
    fixture_safari_header_0();
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 3); }
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 0, start: 22, end: 23 }, Header { name: 9, start: 41, end: 42 }, Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]))) by { reveal(raw_request_spec); }
    assert(allowed(&builtin_intent(), seq![Header { name: 0, start: 22, end: 23 }, Header { name: 9, start: 41, end: 42 }, Header { name: 3, start: 56, end: 57 }, Header { name: 1, start: 67, end: 68 }, Header { name: 8, start: 79, end: 80 }, Header { name: 10, start: 98, end: 99 }, Header { name: 11, start: 117, end: 118 }, Header { name: 4, start: 137, end: 138 }, Header { name: 7, start: 150, end: 151 }, Header { name: 5, start: 170, end: 171 }, Header { name: 2, start: 185, end: 186 }]));
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: firefox.
mod firefox {
    use super::*;
    verus! {
// Reduced firefox fixture: original names/order and single-byte synthetic values.
pub open spec fn fixture_firefox() -> Seq<u8> { b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n"@ }

// Construct header 12 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_11()
    ensures raw_header_spec(fixture_firefox(), 212, 13) == Some(seq![Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    assert(s.subrange(225, 227) =~= b"\r\n"@);
    empty_headers_intent(s, 225, 12);
    assert(raw_prefix_spec(s, 212) == (7u8, 10int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 222) == 223) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(223, 225) =~= b"\r\n"@);
    prepend_header_intent(s, 212, 13, false, 7, 10, 222, 223, seq![]);
    assert(raw_header_spec(s, 212, 13).unwrap() =~= seq![Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 11 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_10()
    ensures raw_header_spec(fixture_firefox(), 193, 14) == Some(seq![Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_11();
    assert(raw_prefix_spec(s, 193) == (12u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 209) == 210) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(210, 212) =~= b"\r\n"@);
    prepend_header_intent(s, 193, 14, false, 12, 16, 209, 210, seq![Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 193, 14).unwrap() =~= seq![Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 10 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_9()
    ensures raw_header_spec(fixture_firefox(), 174, 15) == Some(seq![Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_10();
    assert(raw_prefix_spec(s, 174) == (10u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 190) == 191) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(191, 193) =~= b"\r\n"@);
    prepend_header_intent(s, 174, 15, false, 10, 16, 190, 191, seq![Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 174, 15).unwrap() =~= seq![Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 9 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_8()
    ensures raw_header_spec(fixture_firefox(), 155, 16) == Some(seq![Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_9();
    assert(raw_prefix_spec(s, 155) == (11u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 171) == 172) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(172, 174) =~= b"\r\n"@);
    prepend_header_intent(s, 155, 16, false, 11, 16, 171, 172, seq![Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 155, 16).unwrap() =~= seq![Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 8 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_7()
    ensures raw_header_spec(fixture_firefox(), 136, 17) == Some(seq![Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_8();
    assert(raw_prefix_spec(s, 136) == (9u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 152) == 153) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(153, 155) =~= b"\r\n"@);
    prepend_header_intent(s, 136, 17, false, 9, 16, 152, 153, seq![Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 136, 17).unwrap() =~= seq![Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 7 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_6()
    ensures raw_header_spec(fixture_firefox(), 106, 18) == Some(seq![Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_7();
    assert(raw_prefix_spec(s, 106) == (6u8, 27int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 133) == 134) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(134, 136) =~= b"\r\n"@);
    prepend_header_intent(s, 106, 18, false, 6, 27, 133, 134, seq![Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 106, 18).unwrap() =~= seq![Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 6 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_5()
    ensures raw_header_spec(fixture_firefox(), 91, 19) == Some(seq![Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_6();
    assert(raw_prefix_spec(s, 91) == (2u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 103) == 104) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(104, 106) =~= b"\r\n"@);
    prepend_header_intent(s, 91, 19, false, 2, 12, 103, 104, seq![Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 91, 19).unwrap() =~= seq![Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 5 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_4()
    ensures raw_header_spec(fixture_firefox(), 71, 20) == Some(seq![Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_5();
    assert(raw_prefix_spec(s, 71) == (5u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 88) == 89) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(89, 91) =~= b"\r\n"@);
    prepend_header_intent(s, 71, 20, false, 5, 17, 88, 89, seq![Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 71, 20).unwrap() =~= seq![Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 4 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_3()
    ensures raw_header_spec(fixture_firefox(), 51, 21) == Some(seq![Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_4();
    assert(raw_prefix_spec(s, 51) == (4u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 68) == 69) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(69, 71) =~= b"\r\n"@);
    prepend_header_intent(s, 51, 21, false, 4, 17, 68, 69, seq![Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 51, 21).unwrap() =~= seq![Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 3 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_2()
    ensures raw_header_spec(fixture_firefox(), 40, 22) == Some(seq![Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_3();
    assert(raw_prefix_spec(s, 40) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 48) == 49) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(49, 51) =~= b"\r\n"@);
    prepend_header_intent(s, 40, 22, false, 1, 8, 48, 49, seq![Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 40, 22).unwrap() =~= seq![Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 2 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_1()
    ensures raw_header_spec(fixture_firefox(), 25, 23) == Some(seq![Header { name: 3, start: 37, end: 38 }, Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_2();
    assert(raw_prefix_spec(s, 25) == (3u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 37) == 38) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(38, 40) =~= b"\r\n"@);
    prepend_header_intent(s, 25, 23, false, 3, 12, 37, 38, seq![Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 25, 23).unwrap() =~= seq![Header { name: 3, start: 37, end: 38 }, Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// Construct header 1 and its tail for the firefox witness.
#[verifier::rlimit(20)]
proof fn fixture_firefox_header_0()
    ensures raw_header_spec(fixture_firefox(), 16, 24) == Some(seq![Header { name: 0, start: 22, end: 23 }, Header { name: 3, start: 37, end: 38 }, Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_1();
    assert(raw_prefix_spec(s, 16) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(23, 25) =~= b"\r\n"@);
    prepend_header_intent(s, 16, 24, false, 0, 6, 22, 23, seq![Header { name: 3, start: 37, end: 38 }, Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
    assert(raw_header_spec(s, 16, 24).unwrap() =~= seq![Header { name: 0, start: 22, end: 23 }, Header { name: 3, start: 37, end: 38 }, Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]);
}

// The built-in policy accepts the firefox root header names in captured order.
#[verifier::rlimit(20)]
pub proof fn witness_firefox()
    ensures request_spec(fixture_firefox(), &builtin_intent()).is_some(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_firefox();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nUser-Agent: x\r\nAccept: x\r\nAccept-Language: x\r\nAccept-Encoding: x\r\nConnection: x\r\nUpgrade-Insecure-Requests: x\r\nSec-Fetch-Dest: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-User: x\r\nPriority: x\r\n\r\n");
    fixture_firefox_header_0();
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 3); }
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 0, start: 22, end: 23 }, Header { name: 3, start: 37, end: 38 }, Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]))) by { reveal(raw_request_spec); }
    assert(allowed(&builtin_intent(), seq![Header { name: 0, start: 22, end: 23 }, Header { name: 3, start: 37, end: 38 }, Header { name: 1, start: 48, end: 49 }, Header { name: 4, start: 68, end: 69 }, Header { name: 5, start: 88, end: 89 }, Header { name: 2, start: 103, end: 104 }, Header { name: 6, start: 133, end: 134 }, Header { name: 9, start: 152, end: 153 }, Header { name: 11, start: 171, end: 172 }, Header { name: 10, start: 190, end: 191 }, Header { name: 12, start: 209, end: 210 }, Header { name: 7, start: 222, end: 223 }]));
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: brave.
mod brave {
    use super::*;
    verus! {
// Reduced brave fixture: original names/order and single-byte synthetic values.
pub open spec fn fixture_brave() -> Seq<u8> { b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n"@ }

// Construct header 16 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_15()
    ensures raw_header_spec(fixture_brave(), 280, 9) == Some(seq![Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    assert(s.subrange(300, 302) =~= b"\r\n"@);
    empty_headers_intent(s, 300, 8);
    assert(raw_prefix_spec(s, 280) == (4u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 297) == 298) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(298, 300) =~= b"\r\n"@);
    prepend_header_intent(s, 280, 9, false, 4, 17, 297, 298, seq![]);
    assert(raw_header_spec(s, 280, 9).unwrap() =~= seq![Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 15 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_14()
    ensures raw_header_spec(fixture_brave(), 260, 10) == Some(seq![Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_15();
    assert(raw_prefix_spec(s, 260) == (5u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 277) == 278) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(278, 280) =~= b"\r\n"@);
    prepend_header_intent(s, 260, 10, false, 5, 17, 277, 278, seq![Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 260, 10).unwrap() =~= seq![Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 14 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_13()
    ensures raw_header_spec(fixture_brave(), 241, 11) == Some(seq![Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_14();
    assert(raw_prefix_spec(s, 241) == (9u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 257) == 258) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(258, 260) =~= b"\r\n"@);
    prepend_header_intent(s, 241, 11, false, 9, 16, 257, 258, seq![Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 241, 11).unwrap() =~= seq![Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 13 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_12()
    ensures raw_header_spec(fixture_brave(), 222, 12) == Some(seq![Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_13();
    assert(raw_prefix_spec(s, 222) == (12u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 238) == 239) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(239, 241) =~= b"\r\n"@);
    prepend_header_intent(s, 222, 12, false, 12, 16, 238, 239, seq![Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 222, 12).unwrap() =~= seq![Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 12 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_11()
    ensures raw_header_spec(fixture_brave(), 203, 13) == Some(seq![Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_12();
    assert(raw_prefix_spec(s, 203) == (11u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 219) == 220) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(220, 222) =~= b"\r\n"@);
    prepend_header_intent(s, 203, 13, false, 11, 16, 219, 220, seq![Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 203, 13).unwrap() =~= seq![Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 11 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_10()
    ensures raw_header_spec(fixture_brave(), 184, 14) == Some(seq![Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_11();
    assert(raw_prefix_spec(s, 184) == (10u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 200) == 201) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(201, 203) =~= b"\r\n"@);
    prepend_header_intent(s, 184, 14, false, 10, 16, 200, 201, seq![Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 184, 14).unwrap() =~= seq![Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 10 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_9()
    ensures raw_header_spec(fixture_brave(), 172, 15) == Some(seq![Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_10();
    assert(raw_prefix_spec(s, 172) == (14u8, 9int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 181) == 182) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(182, 184) =~= b"\r\n"@);
    prepend_header_intent(s, 172, 15, false, 14, 9, 181, 182, seq![Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 172, 15).unwrap() =~= seq![Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 9 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_8()
    ensures raw_header_spec(fixture_brave(), 161, 16) == Some(seq![Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_9();
    assert(raw_prefix_spec(s, 161) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 169) == 170) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(170, 172) =~= b"\r\n"@);
    prepend_header_intent(s, 161, 16, false, 1, 8, 169, 170, seq![Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 161, 16).unwrap() =~= seq![Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 8 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_7()
    ensures raw_header_spec(fixture_brave(), 146, 17) == Some(seq![Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_8();
    assert(raw_prefix_spec(s, 146) == (3u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 158) == 159) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(159, 161) =~= b"\r\n"@);
    prepend_header_intent(s, 146, 17, false, 3, 12, 158, 159, seq![Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 146, 17).unwrap() =~= seq![Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 7 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_6()
    ensures raw_header_spec(fixture_brave(), 116, 18) == Some(seq![Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_7();
    assert(raw_prefix_spec(s, 116) == (6u8, 27int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 143) == 144) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(144, 146) =~= b"\r\n"@);
    prepend_header_intent(s, 116, 18, false, 6, 27, 143, 144, seq![Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 116, 18).unwrap() =~= seq![Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 6 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_5()
    ensures raw_header_spec(fixture_brave(), 93, 19) == Some(seq![Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_6();
    assert(name_end(s, 111) == 111) by { reveal(name_end); }
    assert(name_end(s, 110) == 111) by { reveal(name_end); }
    assert(name_end(s, 109) == 111) by { reveal(name_end); }
    assert(name_end(s, 108) == 111) by { reveal(name_end); }
    assert(name_end(s, 107) == 111) by { reveal(name_end); }
    assert(name_end(s, 106) == 111) by { reveal(name_end); }
    assert(name_end(s, 105) == 111) by { reveal(name_end); }
    assert(name_end(s, 104) == 111) by { reveal(name_end); }
    assert(name_end(s, 103) == 111) by { reveal(name_end); }
    assert(name_end(s, 102) == 111) by { reveal(name_end); }
    assert(name_end(s, 101) == 111) by { reveal(name_end); }
    assert(name_end(s, 100) == 111) by { reveal(name_end); }
    assert(name_end(s, 99) == 111) by { reveal(name_end); }
    assert(name_end(s, 98) == 111) by { reveal(name_end); }
    assert(name_end(s, 97) == 111) by { reveal(name_end); }
    assert(name_end(s, 96) == 111) by { reveal(name_end); }
    assert(name_end(s, 95) == 111) by { reveal(name_end); }
    assert(name_end(s, 94) == 111) by { reveal(name_end); }
    assert(name_end(s, 93) == 111) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 93) == (17u8, 20int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 113) == 114) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(114, 116) =~= b"\r\n"@);
    prepend_header_intent(s, 93, 19, false, 17, 20, 113, 114, seq![Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 93, 19).unwrap() =~= seq![Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 5 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_4()
    ensures raw_header_spec(fixture_brave(), 72, 20) == Some(seq![Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_5();
    assert(name_end(s, 88) == 88) by { reveal(name_end); }
    assert(name_end(s, 87) == 88) by { reveal(name_end); }
    assert(name_end(s, 86) == 88) by { reveal(name_end); }
    assert(name_end(s, 85) == 88) by { reveal(name_end); }
    assert(name_end(s, 84) == 88) by { reveal(name_end); }
    assert(name_end(s, 83) == 88) by { reveal(name_end); }
    assert(name_end(s, 82) == 88) by { reveal(name_end); }
    assert(name_end(s, 81) == 88) by { reveal(name_end); }
    assert(name_end(s, 80) == 88) by { reveal(name_end); }
    assert(name_end(s, 79) == 88) by { reveal(name_end); }
    assert(name_end(s, 78) == 88) by { reveal(name_end); }
    assert(name_end(s, 77) == 88) by { reveal(name_end); }
    assert(name_end(s, 76) == 88) by { reveal(name_end); }
    assert(name_end(s, 75) == 88) by { reveal(name_end); }
    assert(name_end(s, 74) == 88) by { reveal(name_end); }
    assert(name_end(s, 73) == 88) by { reveal(name_end); }
    assert(name_end(s, 72) == 88) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 72) == (16u8, 18int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 90) == 91) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(91, 93) =~= b"\r\n"@);
    prepend_header_intent(s, 72, 20, false, 16, 18, 90, 91, seq![Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 72, 20).unwrap() =~= seq![Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 4 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_3()
    ensures raw_header_spec(fixture_brave(), 58, 21) == Some(seq![Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_4();
    assert(name_end(s, 67) == 67) by { reveal(name_end); }
    assert(name_end(s, 66) == 67) by { reveal(name_end); }
    assert(name_end(s, 65) == 67) by { reveal(name_end); }
    assert(name_end(s, 64) == 67) by { reveal(name_end); }
    assert(name_end(s, 63) == 67) by { reveal(name_end); }
    assert(name_end(s, 62) == 67) by { reveal(name_end); }
    assert(name_end(s, 61) == 67) by { reveal(name_end); }
    assert(name_end(s, 60) == 67) by { reveal(name_end); }
    assert(name_end(s, 59) == 67) by { reveal(name_end); }
    assert(name_end(s, 58) == 67) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 58) == (15u8, 11int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 69) == 70) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(70, 72) =~= b"\r\n"@);
    prepend_header_intent(s, 58, 21, false, 15, 11, 69, 70, seq![Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 58, 21).unwrap() =~= seq![Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 3 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_2()
    ensures raw_header_spec(fixture_brave(), 40, 22) == Some(seq![Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_3();
    assert(raw_prefix_spec(s, 40) == (13u8, 15int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 55) == 56) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(56, 58) =~= b"\r\n"@);
    prepend_header_intent(s, 40, 22, false, 13, 15, 55, 56, seq![Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 40, 22).unwrap() =~= seq![Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 2 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_1()
    ensures raw_header_spec(fixture_brave(), 25, 23) == Some(seq![Header { name: 2, start: 37, end: 38 }, Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_2();
    assert(raw_prefix_spec(s, 25) == (2u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 37) == 38) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(38, 40) =~= b"\r\n"@);
    prepend_header_intent(s, 25, 23, false, 2, 12, 37, 38, seq![Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 25, 23).unwrap() =~= seq![Header { name: 2, start: 37, end: 38 }, Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// Construct header 1 and its tail for the brave witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_header_0()
    ensures raw_header_spec(fixture_brave(), 16, 24) == Some(seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_1();
    assert(raw_prefix_spec(s, 16) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(23, 25) =~= b"\r\n"@);
    prepend_header_intent(s, 16, 24, false, 0, 6, 22, 23, seq![Header { name: 2, start: 37, end: 38 }, Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
    assert(raw_header_spec(s, 16, 24).unwrap() =~= seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]);
}

// The built-in policy accepts the brave root header names in captured order.
#[verifier::rlimit(20)]
pub proof fn witness_brave()
    ensures request_spec(fixture_brave(), &builtin_intent()).is_some(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_header_0();
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 3); }
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]))) by { reveal(raw_request_spec); }
    assert(allowed(&builtin_intent(), seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 13, start: 55, end: 56 }, Header { name: 15, start: 69, end: 70 }, Header { name: 16, start: 90, end: 91 }, Header { name: 17, start: 113, end: 114 }, Header { name: 6, start: 143, end: 144 }, Header { name: 3, start: 158, end: 159 }, Header { name: 1, start: 169, end: 170 }, Header { name: 14, start: 181, end: 182 }, Header { name: 10, start: 200, end: 201 }, Header { name: 11, start: 219, end: 220 }, Header { name: 12, start: 238, end: 239 }, Header { name: 9, start: 257, end: 258 }, Header { name: 5, start: 277, end: 278 }, Header { name: 4, start: 297, end: 298 }]));
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: brave cache disabled.
mod brave_cache_disabled {
    use super::*;
    verus! {
// Reduced brave cache disabled fixture: original names/order and single-byte synthetic values.
pub open spec fn fixture_brave_cache_disabled() -> Seq<u8> { b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n"@ }

// Construct header 17 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_16()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 291, 8) == Some(seq![Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    assert(s.subrange(311, 313) =~= b"\r\n"@);
    empty_headers_intent(s, 311, 7);
    assert(raw_prefix_spec(s, 291) == (4u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 308) == 309) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(309, 311) =~= b"\r\n"@);
    prepend_header_intent(s, 291, 8, false, 4, 17, 308, 309, seq![]);
    assert(raw_header_spec(s, 291, 8).unwrap() =~= seq![Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 16 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_15()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 271, 9) == Some(seq![Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_16();
    assert(raw_prefix_spec(s, 271) == (5u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 288) == 289) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(289, 291) =~= b"\r\n"@);
    prepend_header_intent(s, 271, 9, false, 5, 17, 288, 289, seq![Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 271, 9).unwrap() =~= seq![Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 15 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_14()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 252, 10) == Some(seq![Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_15();
    assert(raw_prefix_spec(s, 252) == (9u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 268) == 269) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(269, 271) =~= b"\r\n"@);
    prepend_header_intent(s, 252, 10, false, 9, 16, 268, 269, seq![Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 252, 10).unwrap() =~= seq![Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 14 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_13()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 233, 11) == Some(seq![Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_14();
    assert(raw_prefix_spec(s, 233) == (12u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 249) == 250) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(250, 252) =~= b"\r\n"@);
    prepend_header_intent(s, 233, 11, false, 12, 16, 249, 250, seq![Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 233, 11).unwrap() =~= seq![Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 13 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_12()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 214, 12) == Some(seq![Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_13();
    assert(raw_prefix_spec(s, 214) == (11u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 230) == 231) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(231, 233) =~= b"\r\n"@);
    prepend_header_intent(s, 214, 12, false, 11, 16, 230, 231, seq![Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 214, 12).unwrap() =~= seq![Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 12 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_11()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 195, 13) == Some(seq![Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_12();
    assert(raw_prefix_spec(s, 195) == (10u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 211) == 212) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(212, 214) =~= b"\r\n"@);
    prepend_header_intent(s, 195, 13, false, 10, 16, 211, 212, seq![Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 195, 13).unwrap() =~= seq![Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 11 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_10()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 183, 14) == Some(seq![Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_11();
    assert(raw_prefix_spec(s, 183) == (14u8, 9int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 192) == 193) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(193, 195) =~= b"\r\n"@);
    prepend_header_intent(s, 183, 14, false, 14, 9, 192, 193, seq![Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 183, 14).unwrap() =~= seq![Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 10 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_9()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 172, 15) == Some(seq![Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_10();
    assert(raw_prefix_spec(s, 172) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 180) == 181) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(181, 183) =~= b"\r\n"@);
    prepend_header_intent(s, 172, 15, false, 1, 8, 180, 181, seq![Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 172, 15).unwrap() =~= seq![Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 9 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_8()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 157, 16) == Some(seq![Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_9();
    assert(raw_prefix_spec(s, 157) == (3u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 169) == 170) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(170, 172) =~= b"\r\n"@);
    prepend_header_intent(s, 157, 16, false, 3, 12, 169, 170, seq![Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 157, 16).unwrap() =~= seq![Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 8 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_7()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 127, 17) == Some(seq![Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_8();
    assert(raw_prefix_spec(s, 127) == (6u8, 27int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 154) == 155) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(155, 157) =~= b"\r\n"@);
    prepend_header_intent(s, 127, 17, false, 6, 27, 154, 155, seq![Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 127, 17).unwrap() =~= seq![Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 7 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_6()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 104, 18) == Some(seq![Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_7();
    assert(name_end(s, 122) == 122) by { reveal(name_end); }
    assert(name_end(s, 121) == 122) by { reveal(name_end); }
    assert(name_end(s, 120) == 122) by { reveal(name_end); }
    assert(name_end(s, 119) == 122) by { reveal(name_end); }
    assert(name_end(s, 118) == 122) by { reveal(name_end); }
    assert(name_end(s, 117) == 122) by { reveal(name_end); }
    assert(name_end(s, 116) == 122) by { reveal(name_end); }
    assert(name_end(s, 115) == 122) by { reveal(name_end); }
    assert(name_end(s, 114) == 122) by { reveal(name_end); }
    assert(name_end(s, 113) == 122) by { reveal(name_end); }
    assert(name_end(s, 112) == 122) by { reveal(name_end); }
    assert(name_end(s, 111) == 122) by { reveal(name_end); }
    assert(name_end(s, 110) == 122) by { reveal(name_end); }
    assert(name_end(s, 109) == 122) by { reveal(name_end); }
    assert(name_end(s, 108) == 122) by { reveal(name_end); }
    assert(name_end(s, 107) == 122) by { reveal(name_end); }
    assert(name_end(s, 106) == 122) by { reveal(name_end); }
    assert(name_end(s, 105) == 122) by { reveal(name_end); }
    assert(name_end(s, 104) == 122) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 104) == (17u8, 20int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 124) == 125) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(125, 127) =~= b"\r\n"@);
    prepend_header_intent(s, 104, 18, false, 17, 20, 124, 125, seq![Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 104, 18).unwrap() =~= seq![Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 6 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_5()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 83, 19) == Some(seq![Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_6();
    assert(name_end(s, 99) == 99) by { reveal(name_end); }
    assert(name_end(s, 98) == 99) by { reveal(name_end); }
    assert(name_end(s, 97) == 99) by { reveal(name_end); }
    assert(name_end(s, 96) == 99) by { reveal(name_end); }
    assert(name_end(s, 95) == 99) by { reveal(name_end); }
    assert(name_end(s, 94) == 99) by { reveal(name_end); }
    assert(name_end(s, 93) == 99) by { reveal(name_end); }
    assert(name_end(s, 92) == 99) by { reveal(name_end); }
    assert(name_end(s, 91) == 99) by { reveal(name_end); }
    assert(name_end(s, 90) == 99) by { reveal(name_end); }
    assert(name_end(s, 89) == 99) by { reveal(name_end); }
    assert(name_end(s, 88) == 99) by { reveal(name_end); }
    assert(name_end(s, 87) == 99) by { reveal(name_end); }
    assert(name_end(s, 86) == 99) by { reveal(name_end); }
    assert(name_end(s, 85) == 99) by { reveal(name_end); }
    assert(name_end(s, 84) == 99) by { reveal(name_end); }
    assert(name_end(s, 83) == 99) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 83) == (16u8, 18int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 101) == 102) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(102, 104) =~= b"\r\n"@);
    prepend_header_intent(s, 83, 19, false, 16, 18, 101, 102, seq![Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 83, 19).unwrap() =~= seq![Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 5 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_4()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 69, 20) == Some(seq![Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_5();
    assert(name_end(s, 78) == 78) by { reveal(name_end); }
    assert(name_end(s, 77) == 78) by { reveal(name_end); }
    assert(name_end(s, 76) == 78) by { reveal(name_end); }
    assert(name_end(s, 75) == 78) by { reveal(name_end); }
    assert(name_end(s, 74) == 78) by { reveal(name_end); }
    assert(name_end(s, 73) == 78) by { reveal(name_end); }
    assert(name_end(s, 72) == 78) by { reveal(name_end); }
    assert(name_end(s, 71) == 78) by { reveal(name_end); }
    assert(name_end(s, 70) == 78) by { reveal(name_end); }
    assert(name_end(s, 69) == 78) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 69) == (15u8, 11int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 80) == 81) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(81, 83) =~= b"\r\n"@);
    prepend_header_intent(s, 69, 20, false, 15, 11, 80, 81, seq![Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 69, 20).unwrap() =~= seq![Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 4 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_3()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 51, 21) == Some(seq![Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_4();
    assert(raw_prefix_spec(s, 51) == (13u8, 15int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 66) == 67) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(67, 69) =~= b"\r\n"@);
    prepend_header_intent(s, 51, 21, false, 13, 15, 66, 67, seq![Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 51, 21).unwrap() =~= seq![Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 3 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_2()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 40, 22) == Some(seq![Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_3();
    assert(raw_prefix_spec(s, 40) == (18u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 48) == 49) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(49, 51) =~= b"\r\n"@);
    prepend_header_intent(s, 40, 22, false, 18, 8, 48, 49, seq![Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 40, 22).unwrap() =~= seq![Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 2 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_1()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 25, 23) == Some(seq![Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_2();
    assert(raw_prefix_spec(s, 25) == (2u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 37) == 38) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(38, 40) =~= b"\r\n"@);
    prepend_header_intent(s, 25, 23, false, 2, 12, 37, 38, seq![Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 25, 23).unwrap() =~= seq![Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// Construct header 1 and its tail for the brave cache disabled witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_cache_disabled_header_0()
    ensures raw_header_spec(fixture_brave_cache_disabled(), 16, 24) == Some(seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_1();
    assert(raw_prefix_spec(s, 16) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(23, 25) =~= b"\r\n"@);
    prepend_header_intent(s, 16, 24, false, 0, 6, 22, 23, seq![Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
    assert(raw_header_spec(s, 16, 24).unwrap() =~= seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]);
}

// The built-in policy accepts the brave cache disabled root header names in captured order.
#[verifier::rlimit(20)]
pub proof fn witness_brave_cache_disabled()
    ensures request_spec(fixture_brave_cache_disabled(), &builtin_intent()).is_some(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_cache_disabled();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\n\r\n");
    fixture_brave_cache_disabled_header_0();
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 3); }
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]))) by { reveal(raw_request_spec); }
    assert(allowed(&builtin_intent(), seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }]));
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: brave dnt.
mod brave_dnt {
    use super::*;
    verus! {
// Reduced brave dnt fixture: original names/order and single-byte synthetic values.
pub open spec fn fixture_brave_dnt() -> Seq<u8> { b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n"@ }

// Construct header 18 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_17()
    ensures raw_header_spec(fixture_brave_dnt(), 311, 7) == Some(seq![Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    assert(s.subrange(319, 321) =~= b"\r\n"@);
    empty_headers_intent(s, 319, 6);
    assert(name_end(s, 314) == 314) by { reveal(name_end); }
    assert(name_end(s, 313) == 314) by { reveal(name_end); }
    assert(name_end(s, 312) == 314) by { reveal(name_end); }
    assert(name_end(s, 311) == 314) by { reveal(name_end); }
    assert(s.subrange(314, 316) =~= b": "@);
    assert(raw_prefix_spec(s, 311) == (44u8, 5int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 316) == 317) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(317, 319) =~= b"\r\n"@);
    prepend_header_intent(s, 311, 7, false, 44, 5, 316, 317, seq![]);
    assert(raw_header_spec(s, 311, 7).unwrap() =~= seq![Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 17 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_16()
    ensures raw_header_spec(fixture_brave_dnt(), 291, 8) == Some(seq![Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_17();
    assert(raw_prefix_spec(s, 291) == (4u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 308) == 309) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(309, 311) =~= b"\r\n"@);
    prepend_header_intent(s, 291, 8, false, 4, 17, 308, 309, seq![Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 291, 8).unwrap() =~= seq![Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 16 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_15()
    ensures raw_header_spec(fixture_brave_dnt(), 271, 9) == Some(seq![Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_16();
    assert(raw_prefix_spec(s, 271) == (5u8, 17int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 288) == 289) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(289, 291) =~= b"\r\n"@);
    prepend_header_intent(s, 271, 9, false, 5, 17, 288, 289, seq![Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 271, 9).unwrap() =~= seq![Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 15 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_14()
    ensures raw_header_spec(fixture_brave_dnt(), 252, 10) == Some(seq![Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_15();
    assert(raw_prefix_spec(s, 252) == (9u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 268) == 269) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(269, 271) =~= b"\r\n"@);
    prepend_header_intent(s, 252, 10, false, 9, 16, 268, 269, seq![Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 252, 10).unwrap() =~= seq![Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 14 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_13()
    ensures raw_header_spec(fixture_brave_dnt(), 233, 11) == Some(seq![Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_14();
    assert(raw_prefix_spec(s, 233) == (12u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 249) == 250) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(250, 252) =~= b"\r\n"@);
    prepend_header_intent(s, 233, 11, false, 12, 16, 249, 250, seq![Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 233, 11).unwrap() =~= seq![Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 13 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_12()
    ensures raw_header_spec(fixture_brave_dnt(), 214, 12) == Some(seq![Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_13();
    assert(raw_prefix_spec(s, 214) == (11u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 230) == 231) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(231, 233) =~= b"\r\n"@);
    prepend_header_intent(s, 214, 12, false, 11, 16, 230, 231, seq![Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 214, 12).unwrap() =~= seq![Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 12 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_11()
    ensures raw_header_spec(fixture_brave_dnt(), 195, 13) == Some(seq![Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_12();
    assert(raw_prefix_spec(s, 195) == (10u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 211) == 212) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(212, 214) =~= b"\r\n"@);
    prepend_header_intent(s, 195, 13, false, 10, 16, 211, 212, seq![Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 195, 13).unwrap() =~= seq![Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 11 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_10()
    ensures raw_header_spec(fixture_brave_dnt(), 183, 14) == Some(seq![Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_11();
    assert(raw_prefix_spec(s, 183) == (14u8, 9int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 192) == 193) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(193, 195) =~= b"\r\n"@);
    prepend_header_intent(s, 183, 14, false, 14, 9, 192, 193, seq![Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 183, 14).unwrap() =~= seq![Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 10 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_9()
    ensures raw_header_spec(fixture_brave_dnt(), 172, 15) == Some(seq![Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_10();
    assert(raw_prefix_spec(s, 172) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 180) == 181) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(181, 183) =~= b"\r\n"@);
    prepend_header_intent(s, 172, 15, false, 1, 8, 180, 181, seq![Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 172, 15).unwrap() =~= seq![Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 9 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_8()
    ensures raw_header_spec(fixture_brave_dnt(), 157, 16) == Some(seq![Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_9();
    assert(raw_prefix_spec(s, 157) == (3u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 169) == 170) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(170, 172) =~= b"\r\n"@);
    prepend_header_intent(s, 157, 16, false, 3, 12, 169, 170, seq![Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 157, 16).unwrap() =~= seq![Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 8 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_7()
    ensures raw_header_spec(fixture_brave_dnt(), 127, 17) == Some(seq![Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_8();
    assert(raw_prefix_spec(s, 127) == (6u8, 27int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 154) == 155) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(155, 157) =~= b"\r\n"@);
    prepend_header_intent(s, 127, 17, false, 6, 27, 154, 155, seq![Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 127, 17).unwrap() =~= seq![Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 7 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_6()
    ensures raw_header_spec(fixture_brave_dnt(), 104, 18) == Some(seq![Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_7();
    assert(name_end(s, 122) == 122) by { reveal(name_end); }
    assert(name_end(s, 121) == 122) by { reveal(name_end); }
    assert(name_end(s, 120) == 122) by { reveal(name_end); }
    assert(name_end(s, 119) == 122) by { reveal(name_end); }
    assert(name_end(s, 118) == 122) by { reveal(name_end); }
    assert(name_end(s, 117) == 122) by { reveal(name_end); }
    assert(name_end(s, 116) == 122) by { reveal(name_end); }
    assert(name_end(s, 115) == 122) by { reveal(name_end); }
    assert(name_end(s, 114) == 122) by { reveal(name_end); }
    assert(name_end(s, 113) == 122) by { reveal(name_end); }
    assert(name_end(s, 112) == 122) by { reveal(name_end); }
    assert(name_end(s, 111) == 122) by { reveal(name_end); }
    assert(name_end(s, 110) == 122) by { reveal(name_end); }
    assert(name_end(s, 109) == 122) by { reveal(name_end); }
    assert(name_end(s, 108) == 122) by { reveal(name_end); }
    assert(name_end(s, 107) == 122) by { reveal(name_end); }
    assert(name_end(s, 106) == 122) by { reveal(name_end); }
    assert(name_end(s, 105) == 122) by { reveal(name_end); }
    assert(name_end(s, 104) == 122) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 104) == (17u8, 20int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 124) == 125) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(125, 127) =~= b"\r\n"@);
    prepend_header_intent(s, 104, 18, false, 17, 20, 124, 125, seq![Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 104, 18).unwrap() =~= seq![Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 6 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_5()
    ensures raw_header_spec(fixture_brave_dnt(), 83, 19) == Some(seq![Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_6();
    assert(name_end(s, 99) == 99) by { reveal(name_end); }
    assert(name_end(s, 98) == 99) by { reveal(name_end); }
    assert(name_end(s, 97) == 99) by { reveal(name_end); }
    assert(name_end(s, 96) == 99) by { reveal(name_end); }
    assert(name_end(s, 95) == 99) by { reveal(name_end); }
    assert(name_end(s, 94) == 99) by { reveal(name_end); }
    assert(name_end(s, 93) == 99) by { reveal(name_end); }
    assert(name_end(s, 92) == 99) by { reveal(name_end); }
    assert(name_end(s, 91) == 99) by { reveal(name_end); }
    assert(name_end(s, 90) == 99) by { reveal(name_end); }
    assert(name_end(s, 89) == 99) by { reveal(name_end); }
    assert(name_end(s, 88) == 99) by { reveal(name_end); }
    assert(name_end(s, 87) == 99) by { reveal(name_end); }
    assert(name_end(s, 86) == 99) by { reveal(name_end); }
    assert(name_end(s, 85) == 99) by { reveal(name_end); }
    assert(name_end(s, 84) == 99) by { reveal(name_end); }
    assert(name_end(s, 83) == 99) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 83) == (16u8, 18int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 101) == 102) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(102, 104) =~= b"\r\n"@);
    prepend_header_intent(s, 83, 19, false, 16, 18, 101, 102, seq![Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 83, 19).unwrap() =~= seq![Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 5 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_4()
    ensures raw_header_spec(fixture_brave_dnt(), 69, 20) == Some(seq![Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_5();
    assert(name_end(s, 78) == 78) by { reveal(name_end); }
    assert(name_end(s, 77) == 78) by { reveal(name_end); }
    assert(name_end(s, 76) == 78) by { reveal(name_end); }
    assert(name_end(s, 75) == 78) by { reveal(name_end); }
    assert(name_end(s, 74) == 78) by { reveal(name_end); }
    assert(name_end(s, 73) == 78) by { reveal(name_end); }
    assert(name_end(s, 72) == 78) by { reveal(name_end); }
    assert(name_end(s, 71) == 78) by { reveal(name_end); }
    assert(name_end(s, 70) == 78) by { reveal(name_end); }
    assert(name_end(s, 69) == 78) by { reveal(name_end); }
    assert(raw_prefix_spec(s, 69) == (15u8, 11int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 80) == 81) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(81, 83) =~= b"\r\n"@);
    prepend_header_intent(s, 69, 20, false, 15, 11, 80, 81, seq![Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 69, 20).unwrap() =~= seq![Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 4 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_3()
    ensures raw_header_spec(fixture_brave_dnt(), 51, 21) == Some(seq![Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_4();
    assert(raw_prefix_spec(s, 51) == (13u8, 15int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 66) == 67) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(67, 69) =~= b"\r\n"@);
    prepend_header_intent(s, 51, 21, false, 13, 15, 66, 67, seq![Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 51, 21).unwrap() =~= seq![Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 3 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_2()
    ensures raw_header_spec(fixture_brave_dnt(), 40, 22) == Some(seq![Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_3();
    assert(raw_prefix_spec(s, 40) == (18u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 48) == 49) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(49, 51) =~= b"\r\n"@);
    prepend_header_intent(s, 40, 22, false, 18, 8, 48, 49, seq![Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 40, 22).unwrap() =~= seq![Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 2 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_1()
    ensures raw_header_spec(fixture_brave_dnt(), 25, 23) == Some(seq![Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_2();
    assert(raw_prefix_spec(s, 25) == (2u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 37) == 38) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(38, 40) =~= b"\r\n"@);
    prepend_header_intent(s, 25, 23, false, 2, 12, 37, 38, seq![Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 25, 23).unwrap() =~= seq![Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// Construct header 1 and its tail for the brave dnt witness.
#[verifier::rlimit(20)]
proof fn fixture_brave_dnt_header_0()
    ensures raw_header_spec(fixture_brave_dnt(), 16, 24) == Some(seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_1();
    assert(raw_prefix_spec(s, 16) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 3); }
    assert(s.subrange(23, 25) =~= b"\r\n"@);
    prepend_header_intent(s, 16, 24, false, 0, 6, 22, 23, seq![Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
    assert(raw_header_spec(s, 16, 24).unwrap() =~= seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]);
}

// The built-in policy accepts the brave dnt root header names in captured order.
#[verifier::rlimit(20)]
pub proof fn witness_brave_dnt()
    ensures request_spec(fixture_brave_dnt(), &builtin_intent()).is_some(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = fixture_brave_dnt();
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: x\r\nPragma: x\r\nCache-Control: x\r\nsec-ch-ua: x\r\nsec-ch-ua-mobile: x\r\nsec-ch-ua-platform: x\r\nUpgrade-Insecure-Requests: x\r\nUser-Agent: x\r\nAccept: x\r\nSec-GPC: x\r\nSec-Fetch-Site: x\r\nSec-Fetch-Mode: x\r\nSec-Fetch-User: x\r\nSec-Fetch-Dest: x\r\nAccept-Encoding: x\r\nAccept-Language: x\r\nDNT: x\r\n\r\n");
    fixture_brave_dnt_header_0();
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 3); }
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]))) by { reveal(raw_request_spec); }
    assert(allowed(&builtin_intent(), seq![Header { name: 0, start: 22, end: 23 }, Header { name: 2, start: 37, end: 38 }, Header { name: 18, start: 48, end: 49 }, Header { name: 13, start: 66, end: 67 }, Header { name: 15, start: 80, end: 81 }, Header { name: 16, start: 101, end: 102 }, Header { name: 17, start: 124, end: 125 }, Header { name: 6, start: 154, end: 155 }, Header { name: 3, start: 169, end: 170 }, Header { name: 1, start: 180, end: 181 }, Header { name: 14, start: 192, end: 193 }, Header { name: 10, start: 211, end: 212 }, Header { name: 11, start: 230, end: 231 }, Header { name: 12, start: 249, end: 250 }, Header { name: 9, start: 268, end: 269 }, Header { name: 5, start: 288, end: 289 }, Header { name: 4, start: 308, end: 309 }, Header { name: 44, start: 316, end: 317 }]));
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject headers25.
mod reject_headers25 {
    use super::*;
    verus! {
// The suffix after 24 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_24(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 280, 0, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    reveal(header_spec); reveal(raw_header_spec);
}

// The suffix after 23 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_23(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 269, 1, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_24(legacy);
    assert(header_prefix(s, 269) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 269) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 277) == 278) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 269, 1, legacy);
}

// The suffix after 22 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_22(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 258, 2, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_23(legacy);
    assert(header_prefix(s, 258) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 258) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 266) == 267) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 258, 2, legacy);
}

// The suffix after 21 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_21(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 247, 3, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_22(legacy);
    assert(header_prefix(s, 247) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 247) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 255) == 256) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 247, 3, legacy);
}

// The suffix after 20 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_20(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 236, 4, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_21(legacy);
    assert(header_prefix(s, 236) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 236) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 244) == 245) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 236, 4, legacy);
}

// The suffix after 19 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_19(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 225, 5, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_20(legacy);
    assert(header_prefix(s, 225) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 225) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 233) == 234) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 225, 5, legacy);
}

// The suffix after 18 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_18(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 214, 6, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_19(legacy);
    assert(header_prefix(s, 214) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 214) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 222) == 223) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 214, 6, legacy);
}

// The suffix after 17 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_17(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 203, 7, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_18(legacy);
    assert(header_prefix(s, 203) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 203) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 211) == 212) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 203, 7, legacy);
}

// The suffix after 16 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_16(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 192, 8, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_17(legacy);
    assert(header_prefix(s, 192) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 192) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 200) == 201) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 192, 8, legacy);
}

// The suffix after 15 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_15(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 181, 9, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_16(legacy);
    assert(header_prefix(s, 181) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 181) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 189) == 190) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 181, 9, legacy);
}

// The suffix after 14 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_14(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 170, 10, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_15(legacy);
    assert(header_prefix(s, 170) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 170) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 178) == 179) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 170, 10, legacy);
}

// The suffix after 13 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_13(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 159, 11, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_14(legacy);
    assert(header_prefix(s, 159) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 159) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 167) == 168) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 159, 11, legacy);
}

// The suffix after 12 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_12(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 148, 12, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_13(legacy);
    assert(header_prefix(s, 148) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 148) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 156) == 157) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 148, 12, legacy);
}

// The suffix after 11 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_11(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 137, 13, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_12(legacy);
    assert(header_prefix(s, 137) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 137) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 145) == 146) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 137, 13, legacy);
}

// The suffix after 10 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_10(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 126, 14, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_11(legacy);
    assert(header_prefix(s, 126) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 126) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 134) == 135) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 126, 14, legacy);
}

// The suffix after 9 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_9(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 115, 15, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_10(legacy);
    assert(header_prefix(s, 115) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 115) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 123) == 124) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 115, 15, legacy);
}

// The suffix after 8 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_8(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 104, 16, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_9(legacy);
    assert(header_prefix(s, 104) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 104) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 112) == 113) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 104, 16, legacy);
}

// The suffix after 7 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_7(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 93, 17, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_8(legacy);
    assert(header_prefix(s, 93) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 93) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 101) == 102) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 93, 17, legacy);
}

// The suffix after 6 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_6(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 82, 18, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_7(legacy);
    assert(header_prefix(s, 82) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 82) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 90) == 91) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 82, 18, legacy);
}

// The suffix after 5 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_5(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 71, 19, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_6(legacy);
    assert(header_prefix(s, 71) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 71) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 79) == 80) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 71, 19, legacy);
}

// The suffix after 4 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_4(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 60, 20, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_5(legacy);
    assert(header_prefix(s, 60) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 60) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 68) == 69) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 60, 20, legacy);
}

// The suffix after 3 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_3(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 49, 21, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_4(legacy);
    assert(header_prefix(s, 49) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 49) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 57) == 58) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 49, 21, legacy);
}

// The suffix after 2 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_2(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 38, 22, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_3(legacy);
    assert(header_prefix(s, 38) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 38) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 46) == 47) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 38, 22, legacy);
}

// The suffix after 1 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_1(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 27, 23, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_2(legacy);
    assert(header_prefix(s, 27) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 27) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 35) == 36) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 27, 23, legacy);
}

// The suffix after 0 headers still contains more headers than the remaining budget.
#[verifier::rlimit(20)]
proof fn header25_suffix_0(legacy: bool)
    ensures intent_headers(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, 16, 24, legacy).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_1(legacy);
    assert(header_prefix(s, 16) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(raw_prefix_spec(s, 16) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); }
    assert(value_end(s, 24) == 25) by { reveal_with_fuel(value_end, 3); }
    failed_header_tail(s, 16, 24, legacy);
}

// Every valid policy rejects the headers25 attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_headers25(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    reveal_protocol_literals!();
    let s = b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\nAccept: x\r\n\r\n");
    header25_suffix_0(false); header25_suffix_0(true);
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 3); }
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject cl te.
mod reject_cl_te {
    use super::*;
    verus! {
// Every valid policy rejects the cl te attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_cl_te(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nContent-Length: 0\r\nTransfer-Encoding: chunked\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nContent-Length: 0\r\nTransfer-Encoding: chunked\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nContent-Length: 0\r\nTransfer-Encoding: chunked\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(33, 35) =~= b"\r\n"@);
    assert(s.subrange(61, 63) =~= b"\r\n"@);
    assert(s.subrange(63, 65) =~= b"\r\n"@);
    assert(s.subrange(30, 32) =~= b": "@);
    assert(s.subrange(52, 54) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 30) == 30) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 22) == 30) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 16) == 30) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (19u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 33) == 33) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 32) == 33) by { reveal_with_fuel(value_end, 10); }
    assert(name_end(s, 52) == 52) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 44) == 52) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 36) == 52) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 35) == 52) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 35) == (20u8, 19int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 61) == 61) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 54) == 61) by { reveal_with_fuel(value_end, 10); }
    empty_headers_intent(s, 63, 22);
    prepend_header_intent(s, 35, 23, false, 20, 19, 54, 61, seq![]);
    assert(intent_headers(s, 35, 23, false).unwrap() =~= seq![Header { name: 20, start: 54, end: 61 }]);
    prepend_header_intent(s, 16, 24, false, 19, 16, 32, 33, seq![Header { name: 20, start: 54, end: 61 }]);
    assert(intent_headers(s, 16, 24, false).unwrap() =~= seq![Header { name: 19, start: 32, end: 33 }, Header { name: 20, start: 54, end: 61 }]);
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 19, start: 32, end: 33 }, Header { name: 20, start: 54, end: 61 }]))) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    forbidden_first_header(p, seq![Header { name: 19, start: 32, end: 33 }, Header { name: 20, start: 54, end: 61 }]);
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject te.
mod reject_te {
    use super::*;
    verus! {
// Every valid policy rejects the te attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_te(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(42, 44) =~= b"\r\n"@);
    assert(s.subrange(44, 46) =~= b"\r\n"@);
    assert(s.subrange(33, 35) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 33) == 33) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 25) == 33) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 17) == 33) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 16) == 33) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (20u8, 19int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 42) == 42) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 35) == 42) by { reveal_with_fuel(value_end, 10); }
    empty_headers_intent(s, 44, 23);
    prepend_header_intent(s, 16, 24, false, 20, 19, 35, 42, seq![]);
    assert(intent_headers(s, 16, 24, false).unwrap() =~= seq![Header { name: 20, start: 35, end: 42 }]);
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 20, start: 35, end: 42 }]))) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    forbidden_first_header(p, seq![Header { name: 20, start: 35, end: 42 }]);
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject cl zero.
mod reject_cl_zero {
    use super::*;
    verus! {
// Every valid policy rejects the cl zero attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_cl_zero(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nContent-Length: 0\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nContent-Length: 0\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nContent-Length: 0\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(33, 35) =~= b"\r\n"@);
    assert(s.subrange(35, 37) =~= b"\r\n"@);
    assert(s.subrange(30, 32) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 30) == 30) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 22) == 30) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 16) == 30) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (19u8, 16int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 33) == 33) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 32) == 33) by { reveal_with_fuel(value_end, 10); }
    empty_headers_intent(s, 35, 23);
    prepend_header_intent(s, 16, 24, false, 19, 16, 32, 33, seq![]);
    assert(intent_headers(s, 16, 24, false).unwrap() =~= seq![Header { name: 19, start: 32, end: 33 }]);
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 19, start: 32, end: 33 }]))) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    forbidden_first_header(p, seq![Header { name: 19, start: 32, end: 33 }]);
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject obs space.
mod reject_obs_space {
    use super::*;
    verus! {
// Every valid policy rejects the obs space attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_obs_space(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\n Host: x\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\n Host: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\n Host: x\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(24, 26) =~= b"\r\n"@);
    assert(s.subrange(26, 28) =~= b"\r\n"@);
    assert(s.subrange(21, 23) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 16) == 16) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (44u8, 0int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject obs tab.
mod reject_obs_tab {
    use super::*;
    verus! {
// Every valid policy rejects the obs tab attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_obs_tab(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\n\tHost: x\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\n\tHost: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\n\tHost: x\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(24, 26) =~= b"\r\n"@);
    assert(s.subrange(26, 28) =~= b"\r\n"@);
    assert(s.subrange(21, 23) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 16) == 16) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (44u8, 0int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject bare lf.
mod reject_bare_lf {
    use super::*;
    verus! {
// Every valid policy rejects the bare lf attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_bare_lf(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\nHost: x\n\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\nHost: x\n\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\nHost: x\n\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(19, 21) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject value cr.
mod reject_value_cr {
    use super::*;
    verus! {
// Every valid policy rejects the value cr attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_value_cr(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nHost: x\rx\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nHost: x\rx\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\rx\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(25, 27) =~= b"\r\n"@);
    assert(s.subrange(27, 29) =~= b"\r\n"@);
    assert(s.subrange(20, 22) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_prefix_spec(s, 16) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 23) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (0u8, 6int)) by { reveal(header_prefix); }
    assert(value_end(s, 23) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject value nul.
mod reject_value_nul {
    use super::*;
    verus! {
// Every valid policy rejects the value nul attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_value_nul(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nHost: x\0x\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nHost: x\0x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\0x\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(25, 27) =~= b"\r\n"@);
    assert(s.subrange(27, 29) =~= b"\r\n"@);
    assert(s.subrange(20, 22) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_prefix_spec(s, 16) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 23) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (0u8, 6int)) by { reveal(header_prefix); }
    assert(value_end(s, 23) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject colon space.
mod reject_colon_space {
    use super::*;
    verus! {
// Every valid policy rejects the colon space attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_colon_space(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nHost : x\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nHost : x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost : x\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(24, 26) =~= b"\r\n"@);
    assert(s.subrange(26, 28) =~= b"\r\n"@);
    assert(s.subrange(21, 23) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 20) == 20) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 16) == 20) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (44u8, 0int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject duplicate host.
mod reject_duplicate_host {
    use super::*;
    verus! {
// Every valid policy rejects the duplicate host attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_duplicate_host(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nHost: x\r\nhOsT: x\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nHost: x\r\nhOsT: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nHost: x\r\nhOsT: x\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(23, 25) =~= b"\r\n"@);
    assert(s.subrange(32, 34) =~= b"\r\n"@);
    assert(s.subrange(34, 36) =~= b"\r\n"@);
    assert(s.subrange(20, 22) =~= b": "@);
    assert(s.subrange(29, 31) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_prefix_spec(s, 16) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 23) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(raw_prefix_spec(s, 25) == (0u8, 6int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 32) == 32) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 31) == 32) by { reveal_with_fuel(value_end, 10); }
    empty_headers_intent(s, 34, 22);
    prepend_header_intent(s, 25, 23, false, 0, 6, 31, 32, seq![]);
    assert(intent_headers(s, 25, 23, false).unwrap() =~= seq![Header { name: 0, start: 31, end: 32 }]);
    prepend_header_intent(s, 16, 24, false, 0, 6, 22, 23, seq![Header { name: 0, start: 31, end: 32 }]);
    assert(intent_headers(s, 16, 24, false).unwrap() =~= seq![Header { name: 0, start: 22, end: 23 }, Header { name: 0, start: 31, end: 32 }]);
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 0, start: 22, end: 23 }, Header { name: 0, start: 31, end: 32 }]))) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (0u8, 6int)) by { reveal(header_prefix); }
    assert(value_end(s, 23) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 22) == 23) by { reveal_with_fuel(value_end, 10); }
    assert(header_prefix(s, 25) == (0u8, 6int)) by { reveal(header_prefix); }
    assert(value_end(s, 32) == 32) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 31) == 32) by { reveal_with_fuel(value_end, 10); }
    empty_headers_intent(s, 34, 22);
    prepend_header_intent(s, 25, 23, true, 0, 6, 31, 32, seq![]);
    assert(intent_headers(s, 25, 23, true).unwrap() =~= seq![Header { name: 0, start: 31, end: 32 }]);
    prepend_header_intent(s, 16, 24, true, 0, 6, 22, 23, seq![Header { name: 0, start: 31, end: 32 }]);
    assert(intent_headers(s, 16, 24, true).unwrap() =~= seq![Header { name: 0, start: 22, end: 23 }, Header { name: 0, start: 31, end: 32 }]);
    assert(stage11_request_spec(s) == Some((false, 4, 5, seq![Header { name: 0, start: 22, end: 23 }, Header { name: 0, start: 31, end: 32 }]))) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject duplicate connection.
mod reject_duplicate_connection {
    use super::*;
    verus! {
// Every valid policy rejects the duplicate connection attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_duplicate_connection(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nConnection: x\r\ncOnNeCtIoN: x\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nConnection: x\r\ncOnNeCtIoN: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nConnection: x\r\ncOnNeCtIoN: x\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(29, 31) =~= b"\r\n"@);
    assert(s.subrange(44, 46) =~= b"\r\n"@);
    assert(s.subrange(46, 48) =~= b"\r\n"@);
    assert(s.subrange(26, 28) =~= b": "@);
    assert(s.subrange(41, 43) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_prefix_spec(s, 16) == (2u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 29) == 29) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 28) == 29) by { reveal_with_fuel(value_end, 10); }
    assert(raw_prefix_spec(s, 31) == (2u8, 12int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 44) == 44) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 43) == 44) by { reveal_with_fuel(value_end, 10); }
    empty_headers_intent(s, 46, 22);
    prepend_header_intent(s, 31, 23, false, 2, 12, 43, 44, seq![]);
    assert(intent_headers(s, 31, 23, false).unwrap() =~= seq![Header { name: 2, start: 43, end: 44 }]);
    prepend_header_intent(s, 16, 24, false, 2, 12, 28, 29, seq![Header { name: 2, start: 43, end: 44 }]);
    assert(intent_headers(s, 16, 24, false).unwrap() =~= seq![Header { name: 2, start: 28, end: 29 }, Header { name: 2, start: 43, end: 44 }]);
    assert(raw_request_spec(s) == Some((false, 4, 5, seq![Header { name: 2, start: 28, end: 29 }, Header { name: 2, start: 43, end: 44 }]))) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (2u8, 12int)) by { reveal(header_prefix); }
    assert(value_end(s, 29) == 29) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 28) == 29) by { reveal_with_fuel(value_end, 10); }
    assert(header_prefix(s, 31) == (2u8, 12int)) by { reveal(header_prefix); }
    assert(value_end(s, 44) == 44) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 43) == 44) by { reveal_with_fuel(value_end, 10); }
    empty_headers_intent(s, 46, 22);
    prepend_header_intent(s, 31, 23, true, 2, 12, 43, 44, seq![]);
    assert(intent_headers(s, 31, 23, true).unwrap() =~= seq![Header { name: 2, start: 43, end: 44 }]);
    prepend_header_intent(s, 16, 24, true, 2, 12, 28, 29, seq![Header { name: 2, start: 43, end: 44 }]);
    assert(intent_headers(s, 16, 24, true).unwrap() =~= seq![Header { name: 2, start: 28, end: 29 }, Header { name: 2, start: 43, end: 44 }]);
    assert(stage11_request_spec(s) == Some((false, 4, 5, seq![Header { name: 2, start: 28, end: 29 }, Header { name: 2, start: 43, end: 44 }]))) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject absolute.
mod reject_absolute {
    use super::*;
    verus! {
// Every valid policy rejects the absolute attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_absolute(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET http://x/ HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET http://x/ HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"GET http://x/ HTTP/1.1\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(13, 24) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(22, 24) =~= b"\r\n"@);
    assert(s.subrange(24, 26) =~= b"\r\n"@);
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject star.
mod reject_star {
    use super::*;
    verus! {
// Every valid policy rejects the star attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_star(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET * HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET * HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"GET * HTTP/1.1\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(16, 18) =~= b"\r\n"@);
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject query.
mod reject_query {
    use super::*;
    verus! {
// Every valid policy rejects the query attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_query(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET /? HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET /? HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"GET /? HTTP/1.1\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(6, 17) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(15, 17) =~= b"\r\n"@);
    assert(s.subrange(17, 19) =~= b"\r\n"@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject percent.
mod reject_percent {
    use super::*;
    verus! {
// Every valid policy rejects the percent attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_percent(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET /% HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET /% HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"GET /% HTTP/1.1\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(6, 17) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(15, 17) =~= b"\r\n"@);
    assert(s.subrange(17, 19) =~= b"\r\n"@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject fragment.
mod reject_fragment {
    use super::*;
    verus! {
// Every valid policy rejects the fragment attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_fragment(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET /# HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET /# HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"GET /# HTTP/1.1\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(6, 17) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(15, 17) =~= b"\r\n"@);
    assert(s.subrange(17, 19) =~= b"\r\n"@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject h2 preface.
mod reject_h2_preface {
    use super::*;
    verus! {
// Every valid policy rejects the h2 preface attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_h2_preface(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"@;
    reveal_byteslit(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n");
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(16, 18) =~= b"\r\n"@);
    assert(s.subrange(20, 22) =~= b"\r\n"@);
    assert(s.subrange(22, 24) =~= b"\r\n"@);
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject lower get.
mod reject_lower_get {
    use super::*;
    verus! {
// Every valid policy rejects the lower get attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_lower_get(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"get / HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"get / HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"get / HTTP/1.1\r\n\r\n");
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(16, 18) =~= b"\r\n"@);
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject post.
mod reject_post {
    use super::*;
    verus! {
// Every valid policy rejects the post attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_post(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"POST / HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"POST / HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"POST / HTTP/1.1\r\n\r\n");
    assert(s.subrange(6, 17) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(15, 17) =~= b"\r\n"@);
    assert(s.subrange(17, 19) =~= b"\r\n"@);
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject http12.
mod reject_http12 {
    use super::*;
    verus! {
// Every valid policy rejects the http12 attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_http12(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.2\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.2\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.2\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(16, 18) =~= b"\r\n"@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject http20.
mod reject_http20 {
    use super::*;
    verus! {
// Every valid policy rejects the http20 attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_http20(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/2.0\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/2.0\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/2.0\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(16, 18) =~= b"\r\n"@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject pipeline.
mod reject_pipeline {
    use super::*;
    verus! {
// Every valid policy rejects the pipeline attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_pipeline(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\n\r\nGET / HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\n\r\nGET / HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\n\r\nGET / HTTP/1.1\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(18, 22) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(23, 34) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(16, 18) =~= b"\r\n"@);
    assert(s.subrange(32, 34) =~= b"\r\n"@);
    assert(s.subrange(34, 36) =~= b"\r\n"@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 16) == 16) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (44u8, 0int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject body.
mod reject_body {
    use super::*;
    verus! {
// Every valid policy rejects the body attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_body(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\n\r\nx"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\n\r\nx"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\n\r\nx");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(16, 18) =~= b"\r\n"@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 16) == 16) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (44u8, 0int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject value257.
mod reject_value257 {
    use super::*;
    verus! {
// Every valid policy rejects the value257 attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_value257(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nAccept: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nAccept: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(281, 283) =~= b"\r\n"@);
    assert(s.subrange(283, 285) =~= b"\r\n"@);
    assert(s.subrange(22, 24) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_prefix_spec(s, 16) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 281) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 273) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 265) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 257) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 249) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 241) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 233) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 225) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 217) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 209) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 201) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 193) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 185) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 177) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 169) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 161) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 153) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 145) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 137) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 129) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 121) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 113) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 105) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 97) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 89) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 81) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 73) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 65) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 57) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 49) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 41) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 33) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 25) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 24) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(value_end(s, 281) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 273) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 265) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 257) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 249) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 241) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 233) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 225) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 217) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 209) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 201) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 193) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 185) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 177) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 169) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 161) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 153) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 145) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 137) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 129) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 121) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 113) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 105) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 97) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 89) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 81) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 73) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 65) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 57) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 49) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 41) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 33) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 25) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(value_end(s, 24) == 281) by { reveal_with_fuel(value_end, 10); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject path257.
mod reject_path257 {
    use super::*;
    verus! {
// Every valid policy rejects the path257 attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_path257(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET /xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx HTTP/1.1\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET /xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx HTTP/1.1\r\n\r\n"@;
    reveal_byteslit(b"GET /xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx HTTP/1.1\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(261, 272) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(270, 272) =~= b"\r\n"@);
    assert(s.subrange(272, 274) =~= b"\r\n"@);
    assert(path_end(s, 261) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 253) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 245) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 237) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 229) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 221) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 213) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 205) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 197) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 189) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 181) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 173) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 165) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 157) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 149) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 141) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 133) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 125) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 117) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 109) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 101) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 93) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 85) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 77) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 69) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 61) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 53) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 45) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 37) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 29) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 21) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 13) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 5) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(raw_request_spec(s).is_none()) by { reveal(raw_request_spec); }
    assert(path_end(s, 261) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 253) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 245) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 237) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 229) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 221) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 213) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 205) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 197) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 189) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 181) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 173) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 165) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 157) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 149) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 141) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 133) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 125) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 117) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 109) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 101) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 93) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 85) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 77) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 69) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 61) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 53) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 45) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 37) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 29) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 21) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 13) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 5) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 261) by { reveal_with_fuel(path_end, 10); }
    assert(stage11_request_spec(s).is_none()) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject name65.
mod reject_name65 {
    use super::*;
    verus! {
// Every valid policy rejects the name65 attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_name65(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX: x\r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX: x\r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX: x\r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(84, 86) =~= b"\r\n"@);
    assert(s.subrange(86, 88) =~= b"\r\n"@);
    assert(s.subrange(81, 83) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(name_end(s, 81) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 73) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 65) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 57) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 49) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 41) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 33) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 25) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 17) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(name_end(s, 16) == 81) by { reveal_with_fuel(name_end, 10); }
    assert(raw_prefix_spec(s, 16) == (44u8, 0int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (9u8, 0int)) by { reveal(header_prefix); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}

// Independent proof module: reject empty value.
mod reject_empty_value {
    use super::*;
    verus! {
// Every valid policy rejects the empty value attack witness.
#[verifier::rlimit(20)]
pub proof fn reject_empty_value(p: &Policy)
    requires p.valid(),
    ensures request_spec(b"GET / HTTP/1.1\r\nAccept: \r\n\r\n"@, p).is_none(),
{
    hide(raw_header_spec);
    hide(header_spec);
    hide(raw_prefix_spec);
    hide(header_prefix);
    hide(class_spec);
    hide(raw_request_spec);
    hide(stage11_request_spec);
    hide(request_spec);
    hide(selected_spec);
    hide(name_end);
    hide(value_end);
    hide(path_end);
    literal_bytes();
    let s = b"GET / HTTP/1.1\r\nAccept: \r\n\r\n"@;
    reveal_byteslit(b"GET / HTTP/1.1\r\nAccept: \r\n\r\n");
    assert(s.subrange(0, 4) =~= b"GET "@);
    assert(s.subrange(5, 16) =~= b" HTTP/1.1\r\n"@);
    assert(s.subrange(14, 16) =~= b"\r\n"@);
    assert(s.subrange(24, 26) =~= b"\r\n"@);
    assert(s.subrange(26, 28) =~= b"\r\n"@);
    assert(s.subrange(22, 24) =~= b": "@);
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(raw_prefix_spec(s, 16) == (1u8, 8int)) by { reveal(raw_prefix_spec); reveal(header_prefix); reveal(class_spec); }
    assert(value_end(s, 24) == 24) by { reveal_with_fuel(value_end, 10); }
    assert(raw_header_spec(s, 16, 24) == None) by { reveal(raw_header_spec); }
    assert(raw_request_spec(s) == None) by { reveal(raw_request_spec); }
    assert(path_end(s, 5) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(path_end(s, 4) == 5) by { reveal_with_fuel(path_end, 10); }
    assert(header_prefix(s, 16) == (1u8, 8int)) by { reveal(header_prefix); }
    assert(value_end(s, 24) == 24) by { reveal_with_fuel(value_end, 10); }
    assert(header_spec(s, 16, 24) == None) by { reveal(header_spec); }
    assert(stage11_request_spec(s) == None) by { reveal(stage11_request_spec); }
    reveal(request_spec); reveal(selected_spec);
}
    }
}
