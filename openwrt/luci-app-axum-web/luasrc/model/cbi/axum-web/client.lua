local uci = luci.model.uci.cursor()
local m, e

m = Map("axum-web")
m.title = translate("Axum-Web")
m.description = translate("<a href=\"https://github.com/ykxVK8yL5L/axum-web\" target=\"_blank\">Project GitHub URL</a>")

m:section(SimpleSection).template = "axum-web/axum-web_status"

e = m:section(TypedSection, "server")
e.anonymous = true


host = e:option(Value, "host", translate("Host"))
host.default = "0.0.0.0"
host.datatype = "ipaddr"

port = e:option(Value, "port", translate("Port"))
port.default = "10099"
port.datatype = "port"


webdav_port = e:option(Value, "webdav_port", translate("webdav_port"))
webdav_port.default = "10098"
webdav_port.datatype = "port"

root = e:option(Value, "root", translate("Root Directory"))
root.description = translate("Restrict access to a folder of data, defaults to / which means no restrictions")
root.default = "/root"

database = e:option(Value, "database", translate("database"))
database.default = "/root/axum-web.db"
root.description = translate("Unused")



debug = e:option(Flag, "debug", translate("Debug Mode"))
debug.rmempty = false

return m
