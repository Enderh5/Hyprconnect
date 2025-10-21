package io.github.rodrigoafernandez;

import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;

import com.fasterxml.jackson.databind.ObjectMapper;

import io.github.rodrigoafernandez.protocol.IdentityBody;
import io.github.rodrigoafernandez.protocol.Packet;

/**
 * Hello world!
 *
 */
public class App {
  public static void main(String[] args) throws Exception {
    // Construye el paquete de identidad usando la función generateIdentity
    IdentityBody body = IdentityBody.generateIdentity(); // Si prefieres usar el método generateIdentity() de
    Packet packet = new Packet(body, null);

    // Serializa el objeto a JSON
    ObjectMapper mapper = new ObjectMapper();
    byte[] jsonBytes = mapper.writeValueAsBytes(body);

    // Prepara el broadcast UDP
    DatagramSocket socket = new DatagramSocket();
    socket.setBroadcast(true);

    // Dirección de broadcast y puerto (KDE Connect suele usar 1716)
    InetAddress broadcastAddress = InetAddress.getByName("255.255.255.255");
    int port = 1716;

    DatagramPacket udpPacket = new DatagramPacket(jsonBytes, jsonBytes.length, broadcastAddress, port);

    socket.send(udpPacket);
    System.out.println("Paquete de identidad enviado por broadcast UDP.");
    socket.close();
  }
}
