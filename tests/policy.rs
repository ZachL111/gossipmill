use gossipmill::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 64, capacity: 89, latency: 13, risk: 13, weight: 13 };
    assert_eq!(score(signal), 126);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 66, capacity: 97, latency: 13, risk: 7, weight: 13 };
    assert_eq!(score(signal), 168);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 78, capacity: 74, latency: 16, risk: 10, weight: 6 };
    assert_eq!(score(signal), 128);
    assert_eq!(classify(signal), "review");
}
