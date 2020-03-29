use std::sync::Arc;

#[derive(Debug)]
pub struct TransportConfig {
  pub(crate) snd_wnd: u16,
  pub(crate) rcv_wnd: u16,
  pub(crate) mtu: usize,
  pub(crate) mss: u32,
}

impl Default for TransportConfig {
  fn default() -> Self {
    const KCP_WND_SND: u16 = 32;
    const KCP_WND_RCV: u16 = 128;

    const KCP_MTU_DEF: usize = 1400;

    const KCP_OVERHEAD: usize = 24;

    TransportConfig {
      snd_wnd: KCP_WND_SND,
      rcv_wnd: KCP_WND_RCV,
      mtu: KCP_MTU_DEF,
      mss: (KCP_MTU_DEF - KCP_OVERHEAD) as u32,
    }
  }
}

#[derive(Debug)]
pub struct EndpointConfig {}

#[derive(Debug)]
pub struct ServerConfig {
    pub transport: Arc<TransportConfig>,
}

#[derive(Debug)]
pub struct ClientConfig {
    pub transport: Arc<TransportConfig>,
}
