package io.github.rodrigoafernandez.protocol;

import com.fasterxml.jackson.annotation.JsonAnyGetter;
import com.fasterxml.jackson.annotation.JsonAnySetter;
import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.HashMap;
import java.util.Map;
import java.util.UUID;

public class IdentityBody implements PacketBody {
  @JsonProperty(value = "deviceId", required = true)
  private String deviceId;
  @JsonProperty(value = "deviceName", required = true)
  private String deviceName;
  @JsonProperty(value = "deviceType", required = true)
  private String deviceType;
  @JsonProperty(value = "incomingCapabilities", required = true)
  private String[] incomingCapabilities;
  @JsonProperty(value = "outgoingCapabilities", required = true)
  private String[] outgoingCapabilities;
  @JsonProperty(value = "protocolVersion", required = true)
  private int protocolVersion;

  private Map<String, Object> extra = new HashMap<>();

  public IdentityBody() {
  }

  public IdentityBody(
      String deviceId,
      String deviceName,
      String deviceType,
      String[] incomingCapabilities,
      String[] outgoingCapabilities,
      int protocolVersion) {
    this.deviceId = deviceId;
    this.deviceName = deviceName;
    this.deviceType = deviceType;
    this.incomingCapabilities = incomingCapabilities;
    this.outgoingCapabilities = outgoingCapabilities;
    this.protocolVersion = protocolVersion;
  }

  // Getters y setters estándar

  @JsonAnyGetter
  public Map<String, Object> getExtra() {
    return extra;
  }

  @JsonAnySetter
  public void setExtra(String key, Object value) {
    extra.put(key, value);
  }

  // Método estático para generar una identidad de ejemplo
  public static IdentityBody generateIdentity() {
    UUID uuid = UUID.randomUUID();
    String[] incomingCapabilities = new String[] { "ping" };
    String[] outgoingCapabilities = new String[] { "ping" };
    IdentityBody body = new IdentityBody("_" + uuid.toString().replace('-', '_') + "_", "pcdrdg", "desktop",
        incomingCapabilities,
        outgoingCapabilities, 8);
    body.setExtra("tcpPort", 1716);
    return body;
  }
}
