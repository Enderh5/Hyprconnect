package io.github.rodrigoafernandez;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.io.OutputStreamWriter;
import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.net.ServerSocket;
import java.net.Socket;
import java.net.SocketTimeoutException;
import java.nio.charset.StandardCharsets;
import java.text.spi.DateFormatSymbolsProvider;
import java.util.HashMap;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

import com.fasterxml.jackson.databind.ObjectMapper;

import io.github.rodrigoafernandez.protocol.IdentityBody;
import io.github.rodrigoafernandez.protocol.Packet;
import io.github.rodrigoafernandez.transfer.UdpDiscoveryChannel;

/**
 * Hello world!
 *
 */
public class App {
  static IdentityBody identity;

  public static void main(String[] args) throws Exception {
    identity = IdentityBody.generateIdentity();
    CountDownLatch serverReady = new CountDownLatch(1);

    // Thread pool para manejar conexiones entrantes
    ExecutorService acceptPool = Executors.newCachedThreadPool();
    Thread serverThread = new Thread(() -> {
      try (ServerSocket server = new ServerSocket()) {
        server.setReuseAddress(true);
        server.bind(new InetSocketAddress(1716), 50);
        serverReady.countDown();
        System.out.println("Server listening on port " + 1716);

        while (!Thread.currentThread().isInterrupted()) {
          try {
            Socket client = server.accept(); // bloquea hasta conexión
            System.out.println("Accepted connection from " + client.getRemoteSocketAddress());
            // delegar a handler
            acceptPool.submit(() -> handleClient(client));
          } catch (IOException e) {
            System.err.println("Accept error: " + e.getMessage());
          }
        }

      } catch (Exception e) {
        System.err.println("Error creando websocket: " + e.getMessage());
      }
    });

    serverThread.setDaemon(true);
    serverThread.start();

    // Esperar a que el servidor esté listo antes de enviar broadcast
    serverReady.await();
    sendIdentityBroadcast();

    // Mantener el main vivo para aceptar conexiones; Ctrl-C para salir
    System.out.println("Servidor y broadcast activos. Esperando conexiones...");
    Runtime.getRuntime().addShutdownHook(new Thread(() -> {
      System.out.println("Shutdown requested, terminating...");
      acceptPool.shutdownNow();
    }));

    // Para ejemplo: dormir indefinidamente
    Thread.currentThread().join();
  }

  private static void handleClient(Socket client) {
    try (Socket s = client;
        BufferedReader br = new BufferedReader(new InputStreamReader(s.getInputStream(), StandardCharsets.UTF_8));
        BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(s.getOutputStream(), StandardCharsets.UTF_8))) {

      // Opcional: timeout para no bloquear indefinidamente en readLine()
      s.setSoTimeout(10000); // 10s, ajustar según necesidades

      // Leer la línea completa (terminada en '\n')
      String line = br.readLine();
      if (line == null) {
        System.out.println("[handler] conexión cerrada por el peer sin enviar datos");
        return;
      }
      System.out.println("[handler] Recibido JSON completo: " + line);

      // Enviar respuesta terminada en newline (el remote usa readLine() también)
      IdentityBody body = IdentityBody.generateIdentity();
      Packet packet = new Packet(body, new HashMap<String, Object>());
      ObjectMapper mapper = new ObjectMapper();

      String reply = mapper.writeValueAsString(packet);
      bw.write(reply);
      bw.flush();

      // Indicar que ya no vamos a escribir (opcional, más limpio)
      try {
        s.shutdownOutput();
      } catch (IOException e) {
        // no crítico, continuar intentando cerrar limpiamente
      }

      // Leer cualquier dato adicional que el peer envíe hasta que cierre la conexión.
      // Esto evita dejar bytes sin leer en el buffer del kernel que provoquen RST.
      String extra;
      try {
        while ((extra = br.readLine()) != null) {
          // puedes procesar mensajes adicionales si el protocolo lo requiere
          System.out.println("[handler] Mensaje adicional del peer: " + extra);
        }
      } catch (SocketTimeoutException ste) {
        // tiempo de espera: si no esperas más datos, continuar para cerrar limpiamente
        System.out.println("[handler] Timeout leyendo datos adicionales; cerrando conexión");
      }

      // Al salir del try-with-resources la socket se cerrará limpiamente (FIN), no
      // RST.
    } catch (SocketTimeoutException ste) {
      System.out.println("[handler] Read timeout: " + ste.getMessage());
    } catch (IOException e) {
      System.err.println("[handler] Error manejando cliente: " + e.getMessage());
    }
  }

  private static void sendIdentityBroadcast() {
    UdpDiscoveryChannel channel = new UdpDiscoveryChannel(identity);
    try {
      channel.broadcast();
    } catch (Exception e) {
      System.err.println("Error emitiendo identidad: " + e.getMessage());
    }
    try {
      channel.close();
    } catch (Exception e) {
      System.err.println("Error cerrando el canal UDP: " + e.getMessage());
    }
  }
}
