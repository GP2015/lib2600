use crate::pin::PinSignal;

trait SinglePinRef {
    fn name(&self) -> &str;
    fn signal_possible(&self, signal: PinSignal) -> bool;
    fn high_possible(&self) -> bool;
    fn low_possible(&self) -> bool;
    fn high_z_possible(&self) -> bool;
    fn prev_signal_possible(&self, signal: PinSignal) -> bool;
    fn prev_high_possible(&self) -> bool;
    fn prev_low_possible(&self) -> bool;
    fn prev_high_z_possible(&self) -> bool;
    fn could_read_high(&self) -> bool;
    fn could_read_low(&self) -> bool;
    fn collapsed(&self) -> Option<PinSignal>;
    fn prev_collapsed(&self) -> Option<PinSignal>;
    fn possible_signals(&self) -> Vec<PinSignal>;
    fn prev_possible_signals(&self) -> Vec<PinSignal>;
    fn possible_reads(&self) -> Vec<bool>;
    fn prev_possible_reads(&self) -> Vec<bool>;
}
