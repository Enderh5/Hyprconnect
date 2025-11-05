package io.github.rodrigoafernandez.transfer;

public interface DiscoveryChannel {
  void broadcast() throws Exception; // UDP, Bluetooth beacon, etc.

  void listen(BroadcastListener listener) throws Exception;

  void close() throws Exception;

  interface BroadcastListener {
    void onBroadcastReceived(byte[] data, String senderAddress);
  }
}
