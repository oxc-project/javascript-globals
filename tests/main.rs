use javascript_globals::GLOBALS;

#[test]
fn test() {
    assert!(!GLOBALS["builtin"]["Date"]);
}

#[test]
fn test_audio_worklet() {
    // Test that audioWorklet environment exists
    assert!(GLOBALS.contains_key("audioWorklet"));

    // Test key globals mentioned in the issue
    assert!(!GLOBALS["audioWorklet"]["AudioWorkletGlobalScope"]);
    assert!(!GLOBALS["audioWorklet"]["AudioWorkletProcessor"]);
    assert!(!GLOBALS["audioWorklet"]["WorkletGlobalScope"]);
    assert!(!GLOBALS["audioWorklet"]["currentFrame"]);
    assert!(!GLOBALS["audioWorklet"]["currentTime"]);
    assert!(!GLOBALS["audioWorklet"]["registerProcessor"]);
    assert!(!GLOBALS["audioWorklet"]["sampleRate"]);
}
