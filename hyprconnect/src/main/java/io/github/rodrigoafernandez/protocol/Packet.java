package io.github.rodrigoafernandez.protocol;

import java.time.Instant;
import java.util.Map;

import com.fasterxml.jackson.annotation.JsonProperty;

public class Packet {
  @JsonProperty(value = "id", required = true)
  Instant id;

  @JsonProperty(value = "type", required = true)
  String type;

  @JsonProperty(value = "body", required = true)
  PacketBody body;

  @JsonProperty(value = "payloadSize")
  int payloadSize;

  @JsonProperty(value = "payloadTransferInfo")
  Map<String, Object> payloadTransferInfo;
}
