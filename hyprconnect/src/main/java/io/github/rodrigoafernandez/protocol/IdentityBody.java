package io.github.rodrigoafernandez.protocol;

import java.util.HashMap;
import java.util.Map;
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
}
