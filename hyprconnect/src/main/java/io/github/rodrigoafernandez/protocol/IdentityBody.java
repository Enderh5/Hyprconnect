package io.github.rodrigoafernandez.protocol;

import java.util.HashMap;
import java.util.Map;
import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonAnySetter;
import com.fasterxml.jackson.annotation.JsonProperty;

public record IdentityBody(
    @JsonProperty(value = "deviceId", required = true) String deviceId,
    @JsonProperty(value = "deviceName", required = true) String deviceName,
    @JsonProperty(value = "deviceType", required = true) String deviceType,
    @JsonProperty(value = "incomingCapabilities", required = true) String[] incomingCapabilities,
    @JsonProperty(value = "outgoingCapabilities", required = true) String[] outgoingCapabilities,
    @JsonProperty(value = "protocolVersion", required = true) int protocolVersion,

    Map<String, Object> extra // campos adicionales
) implements PacketBody {

  public IdentityBody {
    if (extra == null) {
      extra = new HashMap<>();
    }
  }

  @JsonAnySetter
  public void addExtra(String key, Object value) {
    extra.put(key, value);
  }

  public static IdentityBody generateIdentity() {
    UUID uuid = UUID.randomUUID();
    String[] incomingCapabilities = new String[] { "ping" };
    String[] outgoingCapabilities = new String[] { "ping" };
    return new IdentityBody(uuid.toString(), "pcdrdg", "desktop", incomingCapabilities, outgoingCapabilities, 8, null);
  }
}
