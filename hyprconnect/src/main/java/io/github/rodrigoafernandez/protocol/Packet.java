package io.github.rodrigoafernandez.protocol;

import java.time.Instant;
import java.util.Map;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.ObjectMapper;

public record Packet(
    @JsonProperty(value = "id", required = true) long id,

    @JsonProperty(value = "type", required = true) String type,

    @JsonProperty(value = "body", required = true) PacketBody body,

    @JsonProperty(value = "payloadSize") int payloadSize,

    @JsonProperty(value = "payloadTransferInfo") Map<String, Object> payloadTransferInfo) {
  public Packet {
    if (!type.matches("^kdeconnect(\\.[a-z_]+)+$")) {
      System.out.println(type);
      System.out.println(type.matches("/^kdeconnect(\\.[a-z_]+)+$/"));
      throw new RuntimeException("Tipo de Paquete inválido");
    }

  }

  public Packet(PacketBody body, Map<String, Object> payloadTransferInfo) {

    String type;
    if (body == null)
      throw new RuntimeException("Se debe aportar un cuerpo al paquete");

    System.out.println(body);
    switch (body) {
      case IdentityBody identity -> {
        type = "kdeconnect.identity";
        break;
      }

      default ->
        throw new RuntimeException("Cuerpo del paquete inválido.");

    }
    ObjectMapper mapper = new ObjectMapper();
    int size = 0;
    try {
      byte[] jsonBytes = mapper.writeValueAsBytes(body);

      size = jsonBytes.length;
    } catch (Exception e) {
      throw new RuntimeException("Error serializando el body", e);
    }

    this(Instant.now().toEpochMilli(), type, body, size, payloadTransferInfo);
  }
}
