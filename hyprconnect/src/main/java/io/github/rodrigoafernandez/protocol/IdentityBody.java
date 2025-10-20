package io.github.rodrigoafernandez.protocol;

import com.fasterxml.jackson.annotation.JsonProperty;

public class KdeConnectIdentity implements PacketBody{
    @JsonProperty(value = "deviceId", required =true )
    public String deviceId;

    @JsonProperty("deviceName")
    public String deviceName;

    @JsonProperty("deviceType")
    public String deviceType;

    @JsonProperty("protocolVersion")
    public int protocolVersion;

    @JsonProperty("tcpPort")
    public int tcpPort;

}
