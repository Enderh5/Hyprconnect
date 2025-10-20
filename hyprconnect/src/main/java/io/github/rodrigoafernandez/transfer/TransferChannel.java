package io.github.rodrigoafernandez.transfer;

// Para la transferencia de datos
public interface TransferChannel {
  void open(String remoteAddress, int port) throws Exception; // Para TCP, Bluetooth, etc.

  void send(byte[] data) throws Exception;

  byte[] receive() throws Exception;

  void close() throws Exception;

  boolean isOpen();

  String getChannelType(); // Ej: "LAN-TCP", "Bluetooth"
}
