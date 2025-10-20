package io.github.rodrigoafernandez.transfer;

public class UdpDiscoveryChannel implements DiscoveryChannel {

  @Override
  public void broadcast(byte[] data) throws Exception {
  }

  @Override
  public void listen(BroadcastListener listener) throws Exception {
    // TODO Auto-generated method stub
    throw new UnsupportedOperationException("Unimplemented method 'listen'");
  }

  @Override
  public void close() throws Exception {
    // TODO Auto-generated method stub
    throw new UnsupportedOperationException("Unimplemented method 'close'");
  }

}
