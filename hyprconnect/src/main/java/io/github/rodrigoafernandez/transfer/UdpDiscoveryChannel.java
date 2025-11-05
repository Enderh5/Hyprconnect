package io.github.rodrigoafernandez.transfer;

import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;
import java.util.HashMap;

import com.fasterxml.jackson.databind.ObjectMapper;

import io.github.rodrigoafernandez.protocol.IdentityBody;
import io.github.rodrigoafernandez.protocol.Packet;

public class UdpDiscoveryChannel implements DiscoveryChannel {
  DatagramSocket socket;
  public DatagramPacket defaultPacket;
  InetAddress bAddress;

  public UdpDiscoveryChannel(IdentityBody body) {
    Packet packet = new Packet(body, new HashMap<String, Object>());
    ObjectMapper mapper = new ObjectMapper();
    try {
      socket = new DatagramSocket();
    } catch (Exception e) {
      throw new RuntimeException("Error creando socket" + e.getMessage(), e);
    }

    int port = 1716;
    try {
      bAddress = InetAddress.getByName("192.168.1.255");
    } catch (Exception e) {
      throw new RuntimeException("Error obteniendo dirección de broadcast: " + e.getMessage(), e);
    }

    try {
      byte[] jsonBytes = mapper.writeValueAsBytes(packet);
      this.defaultPacket = new DatagramPacket(jsonBytes, jsonBytes.length, bAddress, port);
    } catch (Exception e) {
      throw new RuntimeException("Error serializando paquete: " + e.getMessage(), e);
    }
  }

  @Override
  public void broadcast() throws Exception {
    try {
      socket.setBroadcast(true);
      System.out.println("Sending: " + defaultPacket);
      socket.send(defaultPacket);
    } catch (Exception e) {
      throw new RuntimeException("Error al enviar el paquete Broadcast mediante LAN: " + e.getMessage(), e);
    }
  }

  @Override
  public void listen(BroadcastListener listener) throws Exception {
    // TODO Auto-generated method stub
    throw new UnsupportedOperationException("Unimplemented method 'listen'");
  }

  @Override
  public void close() throws Exception {
    socket.close();
  }

}
