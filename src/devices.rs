pub trait Device {
    fn run<C, R>(&mut self, rx_confirm: CCReceiver<C>, tx_report: CCSender<R>);
}
