local yab = require("yab")

local bin = "./out/" .. (yab.os_type() == "windows" and "leiden.exe" or "leiden")
yab.task(yab.find("**.go"), bin, function()
	require("env")
	local build_flags = yab.os_type() == "windows" and '-ldflags "-H=windowsgui"' or '-ldflags "-s -w"'
	os.execute("go build " .. build_flags .. " -o " .. bin .. " ./cmd/leiden")
end)
