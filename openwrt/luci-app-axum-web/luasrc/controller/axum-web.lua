module("luci.controller.axum-web", package.seeall)

function index()
	if not nixio.fs.access("/etc/config/axum-web") then
		return
	end
	entry({"admin", "services", "axum-web"}, alias("admin", "services", "axum-web", "client"),_("Axum-Web"), 10).dependent = true  -- 首页
	entry({"admin", "services", "axum-web", "client"}, cbi("axum-web/client"),_("Settings"), 10).leaf = true  -- 客户端配置
	entry({"admin", "services", "axum-web", "log"}, form("axum-web/log"),_("Log"), 30).leaf = true -- 日志页面

	entry({"admin", "services", "axum-web", "status"}, call("action_status")).leaf = true
	entry({"admin", "services", "axum-web", "logtail"}, call("action_logtail")).leaf = true
end

function action_status()
	local e = {}
	e.running = luci.sys.call("pidof axum-web >/dev/null") == 0
	e.application = luci.sys.exec("axum-web --version")
	luci.http.prepare_content("application/json")
	luci.http.write_json(e)
end

function action_logtail()
	local fs = require "nixio.fs"
	local log_path = "/var/log/axum-web.log"
	local e = {}
	e.running = luci.sys.call("pidof axum-web >/dev/null") == 0
	if fs.access(log_path) then
		e.log = luci.sys.exec("tail -n 100 %s | sed 's/\\x1b\\[[0-9;]*m//g'" % log_path)
	else
		e.log = ""
	end
	luci.http.prepare_content("application/json")
	luci.http.write_json(e)
end
